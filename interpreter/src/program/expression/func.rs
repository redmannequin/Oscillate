use crate::Lexer;
use crate::TokenType;
use crate::Object;
use crate::Container;
use crate::Env;

use crate::traits::LexerTrait;
use crate::traits::ParseTrait;
use crate::traits::EvalTrait;
use crate::traits::NamespaceTrait;

use crate::program::expression::Expression;

use crate::Result;
use crate::error::ParseError;

/// Func
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct Func<O> {
    pub params: Vec<String>,
    pub block: Vec<Expression>,
    pub env: Container<Env<O>>

}

impl<O> Func<O> {
    pub fn new(
        params: Vec<String>, 
        block: Vec<Expression>,
        env: Container<Env<O>>
    ) -> Self { 
        Func { 
            params,
            block,
            env
        } 
    }
}

impl<O> ParseTrait for Func<O> {
    type Lexer = Lexer;
    
    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        unimplemented!()
    }
}

impl EvalTrait<Object> for Func<Object> {

    fn eval(&self, _env: Container<impl NamespaceTrait<Object>>) -> Result<Object> {
        unimplemented!()
    }
}

// #[cfg(test)]
// mod func_tests {
//     use crate::Object;
//     use crate::Container;
//     use crate::Env;
//     use crate::Lexer;

//     use crate::traits::LexerTrait;
//     use crate::traits::ParseTrait;
//     use crate::traits::EvalTrait;

//     use super::Func;

//     use crate::expression::Real;
//     use crate::expression::Expression;

//     #[test]
//     fn array() {
//         let source = "fn test(x,y) { let c = a + b; return c; }";
//         let env = Container::new(Env::new());

//         let mut lexer = Lexer::new(String::from(source));
//         lexer.next();

//         let func = Func::parse(&mut lexer);
//         assert!(func.is_ok(), "Func parse failed: {:?}", func);
//         let func = func.unwrap();

//         let ast = Func::new(
//             vec!["x", "y"],
            
//         );
//         assert_eq!(func, ast);
        
//         let obj = func.eval(env);
//         assert!(obj.is_ok(), "Func eval failed: {:?}", obj);
//         let obj = obj.unwrap();

//         assert_eq!(obj, Object::Array(
//             vec![
//                 Object::Real(1.0),
//                 Object::Real(2.0),
//                 Object::Real(3.0)
//             ]
//         ));
//     }
// }
