use std::result;
use std::error::Error;
use std::fmt;

use parser::error::ParseError;

/// Result for the monkey parser `Error`
pub type Result<T> = result::Result<T, OscillateError>;

/// 
/// ParseError
/// 
#[derive(Debug)]
pub enum OscillateError {
    Parse(ParseError),
    Ops,
}

impl Error for OscillateError {
    fn description(&self) -> &str {
        match *self {
            OscillateError::Parse(ref err) => err.description(),
            OscillateError::Ops => "Ops",
        }
    }
}

impl fmt::Display for OscillateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OscillateError::Parse(ref err) => write!(f, "Parse Error: {}", err),
            OscillateError::Ops => write!(f, "Ops"),
        }
    }
}

impl From<ParseError> for OscillateError {
    fn from(err: ParseError) -> OscillateError {
        OscillateError::Parse(err)
    }
}