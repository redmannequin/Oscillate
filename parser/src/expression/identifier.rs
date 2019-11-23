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
pub struct Identifier {
    pub name: String
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
            _ => return Err(ParseError::ExpectedIdentifier(tok.clone()))
        };
        Ok(Identifier::new(name))
    }
}

impl Eval for Identifier {
    fn eval(&self, env: Container<impl Environment>) -> Result<Object> {
        if let Some(obj) = env.get().get(self.name.as_ref()) {
            return Ok(obj.get().clone());
        }
        Err(ParseError::IdentifierNotFound(self.name.clone()))
    }
}