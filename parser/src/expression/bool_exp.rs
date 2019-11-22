use crate::Lexer;
use crate::Result;
use crate::parse::Parse;
use crate::error::ParseError;
use crate::TokenType;

#[derive(Debug, Clone)]
pub struct Bool {
    value: bool
}

impl Bool {
    pub fn new(value: bool) -> Self { Self { value } }
    pub fn get_vaule(&self) -> bool { self.value }
}

impl Parse for Bool {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        let tok = lexer.curr();
        match tok.token_type {
            TokenType::True => Ok(Bool::new(true)),
            TokenType::False => Ok(Bool::new(false)),
            _ => Err(ParseError::ExpectedBool(tok.clone()))
        }
    }
}