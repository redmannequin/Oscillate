use crate::Lexer;
// use crate::TokenType;

use crate::Result;
// use crate::error::ParseError;

use crate::parse::Parse;

#[derive(Debug, Clone)]
pub struct Use {}

impl Use {
    pub fn new() -> Self { Self {} }
}

impl Parse for Use {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        Ok(Use::new())
    }
}