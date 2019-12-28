use crate::Lexer;
use crate::TokenType;
use crate::Object;
use crate::Container;

use crate::traits::LexerTrait;
use crate::traits::ParseTrait;
use crate::traits::EvalTrait;
use crate::traits::NamespaceTrait;

use crate::Result;
use crate::error::ParseError;

/// Identifier
/// 
#[derive(Debug, Clone, PartialEq)]
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

impl EvalTrait<Object> for Identifier {
    
    fn eval(&self, env: Container<impl NamespaceTrait<Object>>) -> Result<Object> {
        if let Some(obj) = env.get().get(self.name.as_ref()) {
            return Ok(obj.get().clone());
        }
        Err(ParseError::IdentifierNotFound(self.name.clone()))
    }
}

#[cfg(test)]
mod ident_tests {
    use crate::Object;
    use crate::Container;
    use crate::Env;
    use crate::Lexer;

    use crate::traits::LexerTrait;
    use crate::traits::ParseTrait;
    use crate::traits::EvalTrait;
    use crate::traits::NamespaceTrait;

    use super::Identifier;

    #[test]
    fn ident() {
        let source = "a;";
        let env = Container::new(Env::new());
        env.get_mut().set("a", Object::Bool(true));

        let mut lexer = Lexer::new(String::from(source));
        lexer.next();

        let ident = Identifier::parse(&mut lexer);
        assert!(ident.is_ok(), "Identifier parse failed: {:?}", ident);
        let ident = ident.unwrap();

        let ast = Identifier::new(String::from("a"));
        assert_eq!(ident, ast);
        
        let obj = ident.eval(env);
        assert!(obj.is_ok(), "Identifier eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Bool(true));
    }
    
    #[test]
    fn ident_error() {
        let source = "a;";
        let env = Container::new(Env::new());

        let mut lexer = Lexer::new(String::from(source));
        lexer.next();

        let ident = Identifier::parse(&mut lexer);
        assert!(ident.is_ok(), "Identifier parse failed: {:?}", ident);
        let ident = ident.unwrap();
        
        let obj = ident.eval(env);
        assert!(obj.is_err(), "Identifier eval should failed: {:?}", obj);
    }
}


