use crate::Lexer;
use crate::TokenType;
use crate::Result;
use crate::parse::Parse;

use crate::error::ParseError;

#[derive(Debug)]
pub struct Real {
    number: f64
}

impl Real {
    pub fn new(number: f64) -> Self { Self { number } }
    pub fn get_number(&self) -> f64 { self.number }
}

impl Parse for Real {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        let tok = lexer.curr(); 
        let number = match &tok.token_type {
            TokenType::Number(number) => number.parse()?,
            _ => return Err(ParseError::ExpectedIdentifier)
        };

        let tok = lexer.peek();
        if tok.token_type == TokenType::Semicolon {
            lexer.next();
        }

        Ok(Real::new(number))
    }
}