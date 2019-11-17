use std::result;
use std::error::Error;
use std::fmt;

/// Result for the monkey parser `Error`
pub type Result<T> = result::Result<T, ParseError>;

/// 
/// ParseError
/// 
#[derive(Debug)]
pub enum ParseError {
    ExpectedIdentifier,
    ExpectedSemicolon,
    
    ExpectedExpression,

    InvalidSoruce,
    
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::ExpectedIdentifier => "Expected Identifier",
            ParseError::ExpectedSemicolon => "Expected Semicolon",
            ParseError::InvalidSoruce => "Invalid Soruce",
            ParseError::ExpectedExpression => "Expected Expression"
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::ExpectedIdentifier => write!(f, "Expected Identifier"),
            ParseError::ExpectedSemicolon => write!(f, "Expected Semicolon"),
            ParseError::InvalidSoruce => write!(f, "Invalid Soruce"),
            ParseError::ExpectedExpression => write!(f, "Expected Expression")
        }
    }
}
