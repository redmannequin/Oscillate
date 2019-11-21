use crate::Lexer;
use crate::TokenType;

use crate::Result;
use crate::error::ParseError;

use crate::parse::Parse;

#[derive(Debug)]
pub struct Identifier {
    name: String
}

impl Identifier {
    pub fn new(name: String) -> Self { Self { name } }
    pub fn get_name(&self) -> &String { &self.name }
}

impl Parse for Identifier {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        let tok = lexer.curr(); 
        let name = match &tok.token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err(ParseError::ExpectedIdentifier)
        };
        Ok(Identifier::new(name))
    }
}