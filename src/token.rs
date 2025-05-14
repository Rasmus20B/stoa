use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLoc {
    line: u32,
    column: u32,
}

impl SourceLoc {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_val: TokenValue,
    pub source_loc: SourceLoc,
}

impl Token {
    pub fn new(token_val: TokenValue, source_loc: SourceLoc) -> Self {
        Self {
            token_val,
            source_loc,
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {

    Assignment,
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    String,
    True,
    False,

    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    Comma,
    Colon,
    Exclamation,
    EOF,

    Macro,
    MacroParameter,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenValue {
    Assignment,

    Identifier(String),
    IntegerLiteral(u64),
    FloatLiteral(f64),
    String(String),

    True,
    False,

    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    Comma,
    Colon,
    Exclamation,
    EOF,

    Macro,
    MacroParameter(String),
}



impl TokenValue {
    pub fn kind(&self) -> TokenKind {
        match self {
            Self::Assignment =>TokenKind::Assignment,
            Self::Identifier(_) => TokenKind::Identifier,
            Self::IntegerLiteral(_) => TokenKind::IntegerLiteral,
            Self::FloatLiteral(_) => TokenKind::FloatLiteral,
            Self::String(_) =>TokenKind::String,
            Self::True => TokenKind::True,
            Self::False => TokenKind::False,
            Self::OpenBrace =>TokenKind::OpenBrace,
            Self::CloseBrace =>TokenKind::CloseBrace,
            Self::OpenParen => TokenKind::OpenParen,
            Self::CloseParen => TokenKind::CloseParen,
            Self::OpenSquare => TokenKind::OpenSquare,
            Self::CloseSquare => TokenKind::CloseSquare,
            Self::Comma => TokenKind::Comma,
            Self::Colon => TokenKind::Colon,
            Self::Exclamation => TokenKind::Exclamation,
            Self::EOF =>TokenKind::EOF,
            Self::Macro => TokenKind::Macro,
            Self::MacroParameter(_) => TokenKind::MacroParameter,
        }
    } 

    pub fn as_integer(&self) -> Option<u64> {
        match self {
            Self::IntegerLiteral(n) => Some(*n),
            _ => None,
        }
    }
    pub fn as_identifier(&self) -> Option<String> {
        match self {
            Self::Identifier(s) => Some(s.clone()),
            _ => None,
        }
    }
}

pub struct KeywordEntry {
    pub text: &'static str,// Top level objects
    pub val: TokenValue,
}

pub const KEYWORD_MAP: [KeywordEntry; 1] = [
    KeywordEntry {
        text: "macro",
        val: TokenValue::Macro,
    },
];

pub fn match_keyword(word: &str) -> Option<TokenValue> {
    KEYWORD_MAP
        .iter()
        .find(|e| e.text == word)
        .map(|entry| entry.val.clone())
}
