use crate::Lexer;
use crate::TokenType;
use crate::Object;

use crate::traits::Parse;
use crate::traits::Eval;
use crate::traits::Environment;
use crate::traits::Container;

use crate::Result;
use crate::error::ParseError;

#[derive(Debug, Clone)]
pub struct Real {
    pub number: f64
}

impl Real {
    pub fn new(number: f64) -> Self { Self { number } }
}

impl Parse for Real {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        let tok = lexer.curr(); 
        let number = match &tok.token_type {
            TokenType::Number(number) => number.parse()?,
            _ => return Err(ParseError::ExpectedIdentifier(tok.clone()))
        };

        Ok(Real::new(number))
    }
}

impl Eval for Real {
    fn eval(&self, _env: Container<impl Environment>) -> Result<Object> {
        let obj = Object::Real(self.number);
        Ok(obj)
    }
}