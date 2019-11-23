use crate::Lexer;
use crate::TokenType;
use crate::Statement;

use crate::traits::Parse;

use crate::Result;

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