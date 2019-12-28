use crate::Env;
use crate::Object;

use crate::Lexer;
use crate::Container;

use crate::traits::ParseTrait;
use crate::traits::EvalTrait;

use crate::Program;

use crate::Result;

/// Evaluator
/// 
pub struct Evaluator {
    env: Container<Env<Object>>
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Container::new(Env::new())
        }
    }

    pub fn eval(&self, source: String) -> Result<Object> {
        let mut lexer = Lexer::new(source);
        let program = Program::parse(&mut lexer)?;
        let result = program.eval(self.env.clone())?;
        Ok(result)
    }
}

// #[test]
// fn evaluator() {
//     unimplemented!()
// }
