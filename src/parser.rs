use crate::diagnostic::Diagnostic;
use crate::error::{Error, Result};
use crate::token::{SourceLoc, Token, TokenKind, TokenValue};
use crate::keyvalue::{BlockValue, Key, KeyValueBlock, KeyValueEntry};

pub enum TopLevelObject {
    Table {
        id: u32,
        name: String,
        entries: KeyValueBlock,
    },
    Field {
        id: u32,
        name: String,
        entries: KeyValueBlock,
    }
}

struct Parser<'a> {
    stream: std::iter::Peekable<std::slice::Iter<'a, Token>>,
    current: Option<&'a Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            stream: tokens.iter().peekable(),
            current: None,
        }
    }

    pub fn next(&mut self) -> Option<&&Token> {
        self.current = self.stream.next();
        self.current.as_ref()
    }

    pub fn peek(&mut self) -> Option<&&Token> {
        self.stream.peek()
    }

    pub fn expect(&mut self, expected: TokenKind) -> Result<&Token> {
        self.current = match self.stream.next() {
            Some(inner) => Some(inner),
            None => return Err(Error::UnexpectedEOF)
        };

        let current = match self.current {
            Some(inner) => inner,
            None => return Err(Error::UnexpectedEOF)
        };

        self.current = Some(current);
        if current.token_val.kind() == expected {
            return Ok(self.current.unwrap())
        } else {
            return Err(Error::UnexpectedToken(current.clone()))
        }
    }

    pub fn expect_object_id(&mut self) -> Result<u64> {
        let tmp = self.expect(TokenKind::IntegerLiteral)?;
        match tmp.token_val {
            TokenValue::IntegerLiteral(n) => Ok(n),
            _ => unreachable!()
        }
    }

    pub fn expect_identifier(&mut self) -> Result<String> {
        let tmp = self.expect(TokenKind::Identifier)?;
        match &tmp.token_val {
            TokenValue::Identifier(s) => Ok(s.clone()),
            _ => unreachable!()
        }
    }
}

fn parse_macro_key(identifier: String, start_loc: SourceLoc, parser: &mut Parser, diagnostics: &mut Vec<Diagnostic>) -> Result<KeyValueEntry> {
    let next = match parser.next() {
        Some(inner) => inner,
        None => return Err(Error::UnexpectedEOF)
    };

    match &next.token_val {
        TokenValue::Comma => Ok(KeyValueEntry::new(
            Key::MacroValue(identifier),
            start_loc,
            BlockValue::Empty)),
        TokenValue::Assignment => Ok(KeyValueEntry::new(
            Key::MacroValue(identifier),
            start_loc,
            parse_value(parser, diagnostics)?)),
        _ => unimplemented!()
    }
}
fn parse_block(parser: &mut Parser, diagnostics: &mut Vec<Diagnostic>) -> Result<KeyValueBlock> {
    let mut block = KeyValueBlock { entries: vec![] };

    while let Some(token) = parser.next() {
        match &token.token_val {
            TokenValue::Identifier(s) => {
                block.entries.push(parse_identifier_key(s.to_string(), token.source_loc, parser, diagnostics)?);
                let peeked = parser.peek().ok_or(Error::UnexpectedEOF)?;
                if ![TokenValue::Comma, TokenValue::CloseBrace]
                    .contains(&peeked.token_val) {
                    return Err(Error::UnexpectedToken((*peeked).clone()))
                }
            },
            TokenValue::MacroParameter(s) => {
                block.entries.push(parse_macro_key(s.to_string(), token.source_loc, parser, diagnostics)?);
                let peeked = parser.peek().ok_or(Error::UnexpectedEOF)?;
                if ![TokenValue::Comma, TokenValue::CloseBrace]
                    .contains(&peeked.token_val) {
                    return Err(Error::UnexpectedToken((*peeked).clone()))
                }
            },
            TokenValue::Comma => continue,
            TokenValue::CloseBrace => return Ok(block),
            _ => return Err(Error::UnexpectedToken((*token).clone()))
        }
    }
    return Err(Error::UnexpectedEOF)
}

fn parse_value(parser: &mut Parser, diagnostics: &mut Vec<Diagnostic>) -> Result<BlockValue> {
    let token = match parser.next() {
        Some(inner) => inner,
        None => return Err(Error::UnexpectedEOF),
    };

    match &token.token_val {
        TokenValue::OpenBrace => Ok(BlockValue::Block(parse_block(parser, diagnostics)?)),
        TokenValue::IntegerLiteral(n) => Ok(BlockValue::Literal(n.to_string())),
        TokenValue::String(s) => Ok(BlockValue::Literal(s.to_string())),
        TokenValue::MacroParameter(p) => Ok(BlockValue::MacroValue(p.to_string())),
        _ => return Err(Error::UnexpectedToken((*token).clone()))
    }
}

fn parse_identifier_value(identifier: String, start_loc: SourceLoc, parser: &mut Parser, diagnostics: &mut Vec<Diagnostic>) -> Result<BlockValue> {
    let next = match parser.next() {
        Some(inner) => inner,
        None => return Err(Error::UnexpectedEOF)
    };

    match &next.token_val {
        TokenValue::Identifier(s) => parse_identifier_value(identifier + " " +  s, start_loc, parser, diagnostics),
        TokenValue::Comma => return Ok(
            BlockValue::Literal(identifier)
        ),
        _ => unimplemented!()
    }
}

fn parse_identifier_key(identifier: String, start_loc: SourceLoc, parser: &mut Parser, diagnostics: &mut Vec<Diagnostic>) -> Result<KeyValueEntry> {
    let next = match parser.next() {
        Some(inner) => inner,
        None => return Err(Error::UnexpectedEOF)
    };

    match &next.token_val {
        TokenValue::Identifier(s) => parse_identifier_key(identifier + " " + s, start_loc, parser, diagnostics),
        TokenValue::IntegerLiteral(n) => parse_identifier_key(identifier + " " + &n.to_string(), start_loc, parser, diagnostics),
        TokenValue::MacroParameter(p) => {

            Ok(KeyValueEntry::new(
                Key::MacroValue(p.to_string()),
                start_loc,
                match parser.peek().filter(|t| t.token_val.kind() == TokenKind::Assignment) {
                    Some(_) => parse_value(parser, diagnostics)?,
                    None => BlockValue::Empty,
                }
            ))
        },
        TokenValue::Comma => Ok(KeyValueEntry::new(
            Key::MacroValue(identifier),
            start_loc,
            BlockValue::Empty)),
        TokenValue::Assignment => Ok(KeyValueEntry::new(
            Key::Name(identifier),
            start_loc,
            parse_value(parser, diagnostics)?)),
        _ => unimplemented!()
    }
}

fn parse_macro_args(parser: &mut Parser, diagnostics: &mut Vec<Diagnostic>) -> Result<Vec<String>> {
    let mut result = vec![];

    while let Some(token) = parser.peek() {
        match &token.token_val {
            TokenValue::MacroParameter(p) => { 
                result.push(p.to_string());
                parser.next();
                let next = parser.peek();

                if let Some(next) = next {
                    let kind = next.token_val.kind();
                    if kind == TokenKind::CloseParen {
                        parser.next();
                        return Ok(result)
                    } else if kind == TokenKind::Comma {
                        parser.next();
                    } else { return Err(Error::UnexpectedToken((*next).clone())) }
                } else {
                    return Err(Error::UnexpectedEOF)
                }

            }
            TokenValue::CloseParen => { parser.next(); break },
            _ => return Err(Error::UnexpectedToken((*token).clone()))
        }
    }
    Ok(result)
}

fn parse_macro_definition(parser: &mut Parser, diagnostics: &mut Vec<Diagnostic>) -> Result<KeyValueEntry> {
    let location = parser.current.unwrap().source_loc.clone();
    let name = parser.expect_identifier()?;

    let next = match parser.next() {
        Some(inner) => inner,
        None => return Err(Error::UnexpectedEOF)
    };

    let args = if next.token_val.kind() == TokenKind::OpenParen {
        parse_macro_args(parser, diagnostics)?
    } else {
        vec![]
    };

    parser.expect(TokenKind::Assignment)?;

    let next = match parser.next() {
        Some(inner) => inner,
        None => return Err(Error::UnexpectedEOF)
    };

    let value = match &next.token_val {
        TokenValue::OpenBrace => Ok(BlockValue::Block(parse_block(parser, diagnostics)?)),
        TokenValue::IntegerLiteral(n) => Ok(BlockValue::Literal(n.to_string())),
        TokenValue::String(s) => Ok(BlockValue::Literal(s.to_string())),
        TokenValue::MacroParameter(p) => Ok(BlockValue::MacroValue(p.to_string())),
        _ => return Err(Error::UnexpectedToken((*next).clone()))
    }?;

    Ok(KeyValueEntry::new(
        Key::MacroSignature { name, args },
        location,
        value)
    )

}

pub fn parse(tokens: &[Token], diagnostics: &mut Vec<Diagnostic>) -> Result<KeyValueBlock> {
    use crate::token::TokenValue;
    let mut parser = Parser::new(tokens);
    let mut entries = KeyValueBlock::new();

    while let Some(token) = parser.next() {
        let tmp = match &token.token_val {
            TokenValue::Identifier(s) => {
                parse_identifier_key(s.to_string(), token.source_loc, &mut parser, diagnostics)?
            },
            TokenValue::Macro => {
                parse_macro_definition(&mut parser, diagnostics)?
            }
            _ => unreachable!()
        };
        entries.add(tmp);
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use crate::{diagnostic::Diagnostic, token::{SourceLoc, Token, TokenValue}};

    use super::parse;

    #[test]
    fn table_parse() {
        let tokens = vec![
            Token::new(TokenValue::Identifier("Quotes".to_string()), SourceLoc::new(1, 9)),
            Token::new(TokenValue::Assignment, SourceLoc::new(1, 16)),
            Token::new(TokenValue::OpenBrace, SourceLoc::new(1, 18)),
            Token::new(TokenValue::Identifier("id".to_string()), SourceLoc::new(2, 4)),
            Token::new(TokenValue::Assignment, SourceLoc::new(2, 7)),
            Token::new(TokenValue::IntegerLiteral(1), SourceLoc::new(2, 7)),
            Token::new(TokenValue::Comma, SourceLoc::new(2, 9)),
            Token::new(TokenValue::Identifier("type".to_string()), SourceLoc::new(2, 4)),
            Token::new(TokenValue::Assignment, SourceLoc::new(2, 7)),
            Token::new(TokenValue::String("Table".to_string()), SourceLoc::new(2, 7)),
            Token::new(TokenValue::CloseBrace, SourceLoc::new(1, 19)),

            Token::new(TokenValue::Identifier("Machines".to_string()), SourceLoc::new(1, 9)),
            Token::new(TokenValue::Assignment, SourceLoc::new(1, 16)),
            Token::new(TokenValue::OpenBrace, SourceLoc::new(1, 18)),
            Token::new(TokenValue::Identifier("id".to_string()), SourceLoc::new(2, 4)),
            Token::new(TokenValue::Assignment, SourceLoc::new(2, 7)),
            Token::new(TokenValue::IntegerLiteral(2), SourceLoc::new(2, 7)),
            Token::new(TokenValue::Comma, SourceLoc::new(2, 9)),
            Token::new(TokenValue::Identifier("type".to_string()), SourceLoc::new(2, 4)),
            Token::new(TokenValue::Assignment, SourceLoc::new(2, 7)),
            Token::new(TokenValue::String("Table".to_string()), SourceLoc::new(2, 7)),
            Token::new(TokenValue::CloseBrace, SourceLoc::new(1, 19)),

            Token::new(TokenValue::Identifier("Quotes".to_string()), SourceLoc::new(1, 9)),
            Token::new(TokenValue::Identifier("Machines".to_string()), SourceLoc::new(1, 9)),
            Token::new(TokenValue::Assignment, SourceLoc::new(1, 16)),
            Token::new(TokenValue::OpenBrace, SourceLoc::new(1, 18)),
            Token::new(TokenValue::Identifier("id".to_string()), SourceLoc::new(2, 4)),
            Token::new(TokenValue::Assignment, SourceLoc::new(2, 7)),
            Token::new(TokenValue::IntegerLiteral(2), SourceLoc::new(2, 7)),
            Token::new(TokenValue::Comma, SourceLoc::new(2, 9)),
            Token::new(TokenValue::Identifier("type".to_string()), SourceLoc::new(2, 4)),
            Token::new(TokenValue::Assignment, SourceLoc::new(2, 7)),
            Token::new(TokenValue::String("Table".to_string()), SourceLoc::new(2, 7)),
            Token::new(TokenValue::CloseBrace, SourceLoc::new(1, 19)),
        ];

        let mut diagnostics = vec![];
        let store = parse(&tokens, &mut diagnostics).unwrap();
        for entry in store.entries {
            println!("{:?}", entry);
        }
    }
}






