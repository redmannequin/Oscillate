use std::result;
use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;

use crate::Token;

/// Result for the monkey parser `Error`
pub type Result<T> = result::Result<T, ParseError>;

/// 
/// ParseError
/// 
#[derive(Debug)]
pub enum ParseError {
    ExpectedIdentifier(Token),
    ExpectedSemicolon(Token),
    ExpectedBool(Token),
    ExpectedPrefix(Token),
    ExpectedInfix(Token),

    ExpectedOpenRoundBracket(Token),
    ExpectedCloseRoundBracket(Token),

    ExpectedOpenCurlyBracket(Token),
    ExpectedCloseCurlyBracket(Token),

    ExpectedExpression(Token),

    ParseReal,

    InvalidSoruce(Token),
    
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::ExpectedIdentifier(_) => "Expected Identifier",
            ParseError::ExpectedSemicolon(_) => "Expected Semicolon",
            ParseError::InvalidSoruce(_) => "Invalid Soruce",
            ParseError::ExpectedExpression(_) => "Expected Expression",
            ParseError::ExpectedBool(_) => "Expected Bool",
            ParseError::ExpectedPrefix(_) => "Expected Prefix",
            ParseError::ExpectedInfix(_) => "Expected Infix",

            ParseError::ExpectedOpenRoundBracket(_) => "Expected Open Round Bracket",
            ParseError::ExpectedCloseRoundBracket(_) => "Expected Close Round Bracket",
            
            ParseError::ExpectedOpenCurlyBracket(_) => "Expected Open Curly Bracket",
            ParseError::ExpectedCloseCurlyBracket(_) => "Expected Close Curly Bracket",

            ParseError::ParseReal => "Faild to parse Real"
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(_err: ParseFloatError) -> ParseError {
        ParseError::ParseReal
    }
}