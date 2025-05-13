use crate::token::Token;

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // Lexing
    Fs(std::io::Error),
    UnterminatedString,

    // Parser
    UnexpectedToken(Token),
    UnexpectedEOF,
}

impl ::core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
