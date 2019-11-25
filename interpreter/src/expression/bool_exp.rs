use crate::Lexer;
use crate::TokenType;
use crate::Object;
use crate::Container;
use crate::Env;

use crate::traits::LexerTrait;
use crate::traits::ParseTrait;
use crate::traits::EvalTrait;

use crate::Result;
use crate::error::ParseError;

/// Bool
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct Bool {
    pub value: bool
}

impl Bool {
    pub fn new(value: bool) -> Self { Self { value } }
}

impl ParseTrait for Bool {
    type Lexer = Lexer;
    
    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        let tok = lexer.curr();
        match tok.token_type {
            TokenType::True => Ok(Bool::new(true)),
            TokenType::False => Ok(Bool::new(false)),
            _ => Err(ParseError::ExpectedBool(tok.clone()))
        }
    }
}

impl EvalTrait for Bool {
    type Object = Object;
    type Namespace = Env<Object>;

    fn eval(&self, _env: Container<Self::Namespace>) -> Result<Object> {
        Ok(Object::Bool(self.value))
    }
}

#[cfg(test)]
mod bool_tests {
    use crate::Object;
    use crate::Container;
    use crate::Env;
    use crate::Lexer;

    use crate::traits::LexerTrait;
    use crate::traits::ParseTrait;
    use crate::traits::EvalTrait;

    use super::Bool;

    #[test]
    fn bool() {
        let source = "true;";
        let env = Container::new(Env::new());

        let mut lexer = Lexer::new(String::from(source));
        lexer.next();

        let bool_exp = Bool::parse(&mut lexer);
        assert!(bool_exp.is_ok(), "Bool parse failed: {:?}", bool_exp);
        let bool_exp = bool_exp.unwrap();

        let ast = Bool::new(true);
        assert_eq!(bool_exp, ast);
        
        let obj = bool_exp.eval(env);
        assert!(obj.is_ok(), "Bool eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Bool(true));
    }
}
