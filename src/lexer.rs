use std::iter::Peekable;

use crate::{
    diagnostic::Diagnostic,
    error::{Error, Result},
    token::{match_keyword, SourceLoc, Token, TokenValue, KEYWORD_MAP},
};

struct LexIter<'a> {
    input: &'a str,
    line: u32,
    column: u32,
    chars: Peekable<std::str::Chars<'a>>,
}

impl<'a> LexIter<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            line: 1,
            column: 0,
            chars: input.chars().peekable(),
        }
    }
    fn peek(&mut self) -> Option<(char, u32, u32)> {
        if let Some(c) = self.chars.peek() {
            let mut line = self.line;
            let mut column = self.column;
            if *c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            Some((*c, line, column))
        } else {
            None
        }
    }
}

impl<'a> Iterator for LexIter<'a> {
    type Item = (char, u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.chars.next() {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some((c, self.line, self.column))
        } else {
            None
        }
    }
}

pub fn lex(text: &str, diagnostics: &mut Vec<Diagnostic>) -> Result<Vec<Token>> {
    let mut tokens = Vec::<Token>::new();
    let mut iter = LexIter::new(text);
    while let Some((c, line, col)) = iter.next() {
        match c {
            c if c.is_alphabetic() => {
                let mut buffer = String::new();
                buffer.push(c);

                while let Some(&next_c) = iter.chars.peek() {
                    if next_c.is_alphanumeric() {
                        if let Some((c, _, _)) = iter.next() {
                            buffer.push(c);
                        }
                    } else {
                        break;
                    }
                }

                if !buffer.is_empty() {
                    if let Some(keyword) = match_keyword(&buffer) {
                        tokens.push(Token::new(keyword, SourceLoc::new(line, col)));
                    } else {
                        tokens.push(Token::new(
                            TokenValue::Identifier(buffer.trim().to_string()),
                            SourceLoc::new(line, col),
                        ));
                    }
                }
            }
            d if d.is_numeric() => {
                let mut buffer = String::new();
                buffer.push(d);
                let mut is_float = false;

                while let Some(&next_c) = iter.chars.peek() {
                    if next_c.is_ascii_digit() {
                        buffer.push(next_c);
                        iter.next();
                    } else {
                        if next_c == '.' {
                            // If we already have a dot
                            is_float = true;
                            buffer.push(next_c);
                            iter.next();
                        } else {
                            break;
                        }
                    }
                }

                let value = if is_float {
                    TokenValue::FloatLiteral(buffer.parse::<f64>().unwrap())
                } else {
                    TokenValue::IntegerLiteral(buffer.parse::<u64>().unwrap())
                };

                tokens.push(Token::new(value, SourceLoc::new(line, col)));
            }
            '"' => {
                // TODO: Proper string parsing with escapes
                let mut buffer = String::new();
                while let Some(next_c) = iter.chars.peek() {
                    if *next_c == '"' {
                        iter.next();
                        break;
                    } 
                    buffer.push(*next_c);
                    iter.next();
                }
                tokens.push(Token::new(
                    TokenValue::String(buffer),
                    SourceLoc::new(line, col),
                ));
            }
            '=' => tokens.push(Token::new(
                TokenValue::Assignment,
                SourceLoc::new(line, col),
            )),
            '{' => tokens.push(Token::new(TokenValue::OpenBrace, SourceLoc::new(line, col))),
            '}' => tokens.push(Token::new(
                TokenValue::CloseBrace,
                SourceLoc::new(line, col),
            )),
            ',' => tokens.push(Token::new(TokenValue::Comma, SourceLoc::new(line, col))),
            '(' => tokens.push(Token::new(TokenValue::OpenParen, SourceLoc::new(line, col))),
            ')' => tokens.push(Token::new(TokenValue::CloseParen, SourceLoc::new(line, col))),
            '$' => {
                let mut buffer = String::new();
                let Some(next_c) = iter.chars.peek() else {
                    return Err(Error::UnexpectedEOF)
                };

                if !next_c.is_alphabetic() { return Err(Error::MalformedMacroParameterName) }

                while let Some(next_c) = iter.chars.peek() {
                    if !next_c.is_alphanumeric() {
                        break;
                    }
                    buffer.push(*next_c);
                    iter.next();
                }

                tokens.push(Token::new(TokenValue::MacroParameter(buffer), SourceLoc::new(line, col)));
            }
            '@' =>  {
                let mut buffer = String::new();
                let Some(next_c) = iter.chars.peek() else {
                    return Err(Error::UnexpectedEOF)
                };

                if !next_c.is_alphabetic() { return Err(Error::MalformedMacroParameterName) }

                while let Some(next_c) = iter.chars.peek() {
                    if !next_c.is_alphanumeric() {
                        break;
                    }
                    buffer.push(*next_c);
                    iter.next();
                }
                tokens.push(Token::new(TokenValue::MacroCall(buffer), SourceLoc::new(line, col)));
            },
            _ => {}
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::token::TokenValue;

    use super::lex;

    #[test]
    fn multi_word_identifier() {
        let text = "table 1 Quotes Machines 2";
        let mut diags = vec![];
        let tokens = lex(text, &mut diags).unwrap();
        assert_eq!(tokens[0].token_val, TokenValue::Identifier("table".to_string()));
        assert_eq!(tokens[1].token_val, TokenValue::IntegerLiteral(1));
        assert_eq!(
            tokens[2].token_val,
            TokenValue::Identifier(String::from("Quotes"))
        );
        assert_eq!(
            tokens[3].token_val,
            TokenValue::Identifier(String::from("Machines"))
        );
        assert_eq!(
            tokens[4].token_val,
            TokenValue::IntegerLiteral(2)
        );
    }

    #[test]
    fn multi_word_identifier_after_keyword() {
        let text = "table \"Quotes\" Machines 1.64 Quotes Machines 2";
        let mut diags = vec![];
        let tokens = lex(text, &mut diags).unwrap();
        assert_eq!(tokens[0].token_val, TokenValue::Identifier("table".to_string()));
        assert_eq!(
            tokens[1].token_val,
            TokenValue::String("Quotes".to_string())
        );
        assert_eq!(
            tokens[2].token_val,
            TokenValue::Identifier("Machines".to_string())
        );
        assert_eq!(
            tokens[3].token_val,
            TokenValue::FloatLiteral(1.64)
        );
        assert_eq!(
            tokens[4].token_val,
            TokenValue::Identifier("Quotes".to_string())
        );
        assert_eq!(
            tokens[5].token_val,
            TokenValue::Identifier("Machines".to_string())
        );
        assert_eq!(
            tokens[6].token_val,
            TokenValue::IntegerLiteral(2)
        );
    }

    #[test]
    fn keyword_repeated() {
        let text = "table table table 1 Quotes = {}";
        let mut diags = vec![];
        let tokens = lex(text, &mut diags).unwrap();
        assert_eq!(tokens[0].token_val, TokenValue::Identifier("table".to_string()));
        assert_eq!(tokens[1].token_val, TokenValue::Identifier("table".to_string()));
        assert_eq!(tokens[2].token_val, TokenValue::Identifier("table".to_string()));
        assert_eq!(tokens[3].token_val, TokenValue::IntegerLiteral(1));
        assert_eq!(
            tokens[4].token_val,
            TokenValue::Identifier(String::from("Quotes"))
        );
        assert_eq!(tokens[5].token_val, TokenValue::Assignment);
        assert_eq!(tokens[6].token_val, TokenValue::OpenBrace);
        assert_eq!(tokens[7].token_val, TokenValue::CloseBrace);
    }
}
