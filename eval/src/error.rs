use std::result;
use std::error::Error;
use std::fmt;

use parser::error::ParseError;

/// Result for the monkey parser `Error`
pub type Result<T> = result::Result<T, EvalError>;

/// 
/// ParseError
/// 
#[derive(Debug)]
pub enum EvalError {
    Parse(ParseError),
    IdentifierNotFound(String),
    Ops,
}

impl Error for EvalError {
    fn description(&self) -> &str {
        match *self {
            EvalError::Parse(ref err) => err.description(),
            EvalError::IdentifierNotFound(_) => "Identifier Not Found",
            EvalError::Ops => "Ops",
        }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<ParseError> for EvalError {
    fn from(err: ParseError) -> EvalError {
        EvalError::Parse(err)
    }
}