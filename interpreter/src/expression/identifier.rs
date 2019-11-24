use crate::Lexer;
use crate::TokenType;
use crate::Object;
use crate::Container;
use crate::Env;

use crate::traits::LexerTrait;
use crate::traits::ParseTrait;
use crate::traits::EvalTrait;
use crate::traits::NamespaceTrait;

use crate::Result;
use crate::error::ParseError;

/// Identifier
/// 
#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String
}

impl Identifier {
    pub fn new(name: String) -> Self { Self { name } }
    pub fn get_name(&self) -> &String { &self.name }
}

impl ParseTrait for Identifier {
    type Lexer = Lexer;

    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        let tok = lexer.curr(); 
        let name = match &tok.token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err(ParseError::ExpectedIdentifier(tok.clone()))
        };
        Ok(Identifier::new(name))
    }
}

impl EvalTrait for Identifier {
    type Object = Object;
    type Namespace = Env<Object>;

    fn eval(&self, env: Container<Self::Namespace>) -> Result<Object> {
        if let Some(obj) = env.get().get(self.name.as_ref()) {
            return Ok(obj.get().clone());
        }
        Err(ParseError::IdentifierNotFound(self.name.clone()))
    }
}

#[test]
fn ident() {
    unimplemented!()
}