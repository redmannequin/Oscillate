use crate::Lexer;
use crate::TokenType;

use crate::Result;
use crate::error::ParseError;

use crate::parse::Parse;

#[derive(Debug)]
pub struct Set {}

impl Set {
    pub fn new() -> Self { Self {} }
}

impl Parse for Set {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        Ok(Set::new())
    }
}