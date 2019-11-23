use crate::Env;
use crate::Object;

use crate::Lexer;
use crate::Parser;

use crate::traits::Eval;
use crate::traits::Container;

use crate::Result;

/// Evaluator
/// 
pub struct Evaluator {
    env: Container<Env>
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Container::new(Env::new())
        }
    }

    pub fn eval(&self, source: String) -> Result<Object> {
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse()?;

        let mut result = Object::Null;
        for statement in program {
            result = statement.eval(self.env.clone())?;
        }
        Ok(result)
    }
}