use crate::Lexer;
use crate::TokenType;
use crate::Statement;

use crate::traits::ParseTrait;
use crate::traits::LexerTrait;

use crate::Result;
use crate::error::ParseError;

pub type Program = Vec<Statement>;

/// Parser
/// 
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

pub fn expect_curr(lexer: &mut Lexer, tok_type: TokenType, err: ParseError) -> Result<()> {
    let token = lexer.curr();
    if token.token_type == tok_type {
        Ok(())
    } else {
        Err(err)
    }
}

pub fn expect_peek(lexer: &mut Lexer, tok_type: TokenType, err: ParseError) -> Result<()> {
    let token = lexer.next();
    if token.token_type == tok_type {
        Ok(())
    } else {
        Err(err)
    }
}

pub fn on_semicolon(lexer: &mut Lexer) -> Result<()> {
    let tok = lexer.curr();
    if tok.token_type == TokenType::Semicolon {
        Ok(())
    } else {
        Err(ParseError::ExpectedSemicolon(tok.clone()))
    }
}

// #[test]
// fn parser() {
//     unimplemented!()
// }
