use crate::Lexer;
use crate::TokenType;
use crate::Result;
use crate::parse::Parse;

use crate::error::ParseError;

#[derive(Debug, Clone)]
pub struct Real {
    number: f64
}

impl Real {
    pub fn new(number: f64) -> Self { Self { number } }
    pub fn get_number(&self) -> f64 { self.number }
}

impl Parse for Real {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        let tok = &lexer.curr().token_type; 
        let number = match tok {
            TokenType::Number(number) => number.parse()?,
            _ => return Err(ParseError::ExpectedIdentifier)
        };

        Ok(Real::new(number))
    }
}