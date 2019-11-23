
pub mod expression;
pub use expression::Expression;

pub mod statement;
pub use statement::Statement;

mod env;
pub use env::Env;

pub mod error;
pub use error::Result;

mod lexer;
pub use lexer::Lexer;

mod object;
pub use object::Object;

pub mod token;
pub use token::Token;
pub use token::TokenType;

pub mod traits;
use traits::Parse;
use traits::Eval;
use traits::Environment;
use traits::Container;

pub type Program = Vec<Statement>;

pub struct Parser {
    lexer: Lexer
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            lexer
        }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut program = Program::new();
        loop {
            let tok = self.lexer.next().token_type.clone();
            let stmt = match tok {
                TokenType::EOF => break,
                TokenType::Illegal => break,
                _ => Statement::parse(&mut self.lexer)?
            };
            program.push(stmt);
        }
        Ok(program)
    }

}

pub fn eval(program: &Program, env: Container<impl Environment>) -> Result<Object> {
    let mut result = Object::Null;
    for statement in program {
        result = statement.eval(env.clone())?;
    }
    Ok(result)
}