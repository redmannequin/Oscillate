use crate::Lexer;
use crate::TokenType;
use crate::Object;
use crate::Container;
use crate::Env;

use crate::traits::LexerTrait;
use crate::traits::ParseTrait;
use crate::traits::EvalTrait;

use crate::expression::Expression;

use crate::Result;
use crate::error::ParseError;

/// Array
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    pub value: Vec<Expression>
}

impl Array {
    pub fn new(value: Vec<Expression>) -> Self { Self { value } }
}

impl ParseTrait for Array {
    type Lexer = Lexer;
    
    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        let tok = lexer.curr();
        if TokenType::OpenSquareBracket != tok.token_type {
            return Err(ParseError::Ops);
        }
        lexer.next();

        let mut value = Vec::new();
        loop {
            let exp = Expression::parse(lexer)?;
            value.push(exp);
            let tok = lexer.next();
            if TokenType::CloseSquareBracket == tok.token_type {
                break;
            } else if TokenType::Comma != tok.token_type {
                return Err(ParseError::Ops);
            } else {
                lexer.next();
            }
        }

        Ok(Array::new(value))
    }
}

impl EvalTrait for Array {
    type Object = Object;
    type Namespace = Env<Object>;

    fn eval(&self, env: Container<Self::Namespace>) -> Result<Object> {
        let mut arr = Vec::new();
        for item in self.value.iter() {
            let item = item.eval(env.clone())?;
            arr.push(item);
        }
        Ok(Object::Array(arr))
    }
}

#[cfg(test)]
mod array_tests {
    use crate::Object;
    use crate::Container;
    use crate::Env;
    use crate::Lexer;

    use crate::traits::LexerTrait;
    use crate::traits::ParseTrait;
    use crate::traits::EvalTrait;

    use super::Array;

    use crate::expression::Real;
    use crate::expression::Expression;

    #[test]
    fn array() {
        let source = "[1,2,3];";
        let env = Container::new(Env::new());

        let mut lexer = Lexer::new(String::from(source));
        lexer.next();

        let array = Array::parse(&mut lexer);
        assert!(array.is_ok(), "Array parse failed: {:?}", array);
        let array = array.unwrap();

        let ast = Array::new(
            vec![
                Expression::Real(Real::new(1.0)), 
                Expression::Real(Real::new(2.0)),
                Expression::Real(Real::new(3.0))
            ]
        );
        assert_eq!(array, ast);
        
        let obj = array.eval(env);
        assert!(obj.is_ok(), "Array eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Array(
            vec![
                Object::Real(1.0),
                Object::Real(2.0),
                Object::Real(3.0)
            ]
        ));
    }
}
