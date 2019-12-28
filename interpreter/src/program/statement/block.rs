use crate::TokenType;
use crate::Lexer;

use crate::program::Statement;

use crate::Result;
use crate::error::ParseError;

use crate::traits::ParseTrait;
use crate::traits::LexerTrait;

/// Block
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub block: Vec<Statement>
}

impl Block {
    pub fn new(block: Vec<Statement>) -> Self {
        Self { block }
    }
}

impl ParseTrait for Block {
    type Lexer = Lexer;

    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        let mut block = Vec::new();
        loop {
            let tok = lexer.next().token_type.clone();
            let stmt = match tok {
                TokenType::EOF => return Err(ParseError::Ops),
                TokenType::Illegal => return Err(ParseError::Ops),
                TokenType::CloseCurlyBracket => break,
                _ => Statement::parse(lexer)? 
            };
            block.push(stmt);
        }
        Ok(Block::new(block))
    }
}