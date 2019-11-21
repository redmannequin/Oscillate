
pub mod token;
pub use token::Token;
pub use token::TokenType;

mod lexer;
pub use lexer::Lexer;

mod parse;
pub use parse::Parse;

pub mod error;
pub use error::Result;

pub mod expression;
pub mod statement;

use statement::Statement;

pub struct Parser {
    lexer: Lexer
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            lexer
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>> {
        let mut program = Vec::new();
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