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

/// Real
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct Real {
    pub number: f64
}

impl Real {
    pub fn new(number: f64) -> Self { Self { number } }
}

impl ParseTrait for Real {
    type Lexer = Lexer;

    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        let tok = lexer.curr(); 
        let number = match &tok.token_type {
            TokenType::Number(number) => number.parse()?,
            _ => return Err(ParseError::ExpectedIdentifier(tok.clone()))
        };

        Ok(Real::new(number))
    }
}

impl EvalTrait for Real {
    type Object = Object;
    type Namespace = Env<Object>;

    fn eval(&self, _env: Container<Self::Namespace>) -> Result<Object> {
        let obj = Object::Real(self.number);
        Ok(obj)
    }
}

#[cfg(test)]
mod real_tests {
    use crate::Object;
    use crate::Container;
    use crate::Env;
    use crate::Lexer;

    use crate::traits::LexerTrait;
    use crate::traits::ParseTrait;
    use crate::traits::EvalTrait;

    use super::Real;

    #[test]
    fn real_with_decimal() {
        let source = "0.5;";
        let env = Container::new(Env::new());

        let mut lexer = Lexer::new(String::from(source));
        lexer.next();

        let real = Real::parse(&mut lexer);
        assert!(real.is_ok(), "Real parse failed: {:?}", real);
        let real = real.unwrap();
        
        let obj = real.eval(env);
        assert!(obj.is_ok(), "Real eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Real(0.5));
    }

    #[test]
    fn real_without_decimal() {
        let source = "5;";
        let env = Container::new(Env::new());

        let mut lexer = Lexer::new(String::from(source));
        lexer.next();

        let real = Real::parse(&mut lexer);
        assert!(real.is_ok(), "Real parse failed: {:?}", real);
        let real = real.unwrap();
        
        let obj = real.eval(env);
        assert!(obj.is_ok(), "Real eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Real(5.0));
    }
}