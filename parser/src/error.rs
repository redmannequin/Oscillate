use std::result;
use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;

/// Result for the monkey parser `Error`
pub type Result<T> = result::Result<T, ParseError>;

/// 
/// ParseError
/// 
#[derive(Debug)]
pub enum ParseError {
    ExpectedIdentifier,
    ExpectedSemicolon,
    ExpectedBool,
    ExpectedPrefix,
    ExpectedInfix,

    ExpectedOpenRoundBracket,
    ExpectedCloseRoundBracket,

    ExpectedOpenCurlyBracket,
    ExpectedCloseCurlyBracket,

    ExpectedExpression,

    ParseReal,

    InvalidSoruce,
    
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::ExpectedIdentifier => "Expected Identifier",
            ParseError::ExpectedSemicolon => "Expected Semicolon",
            ParseError::InvalidSoruce => "Invalid Soruce",
            ParseError::ExpectedExpression => "Expected Expression",
            ParseError::ExpectedBool => "Expected Bool",
            ParseError::ExpectedPrefix => "Expected Prefix",
            ParseError::ExpectedInfix => "Expected Infix",

            ParseError::ExpectedOpenRoundBracket => "Expected Open Round Bracket",
            ParseError::ExpectedCloseRoundBracket => "Expected Close Round Bracket",
            
            ParseError::ExpectedOpenCurlyBracket => "Expected Open Curly Bracket",
            ParseError::ExpectedCloseCurlyBracket => "Expected Close Curly Bracket",

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