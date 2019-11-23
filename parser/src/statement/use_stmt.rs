use crate::Lexer;
// use crate::TokenType;

use crate::Result;
// use crate::error::ParseError;

use crate::traits::Parse;

#[derive(Debug, Clone)]
pub struct Use {}

impl Use {
    pub fn new() -> Self { Self {} }
}

impl Parse for Use {
    fn parse(_lexer: &mut Lexer) -> Result<Self> {
        Ok(Use::new())
    }
}