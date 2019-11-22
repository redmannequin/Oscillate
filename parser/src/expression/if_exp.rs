use crate::Lexer;
use crate::Result;
use crate::parse::Parse;
use crate::error::ParseError;
use crate::TokenType;

use crate::expression::Expression;
use crate::statement::Statement;

#[derive(Debug, Clone)]
pub struct If {
    cond_blocks: Vec<(Box<Expression>, Vec<Statement>)>,
    otherwise: Option<Vec<Statement>>
}

impl If {

    pub fn new(
        cond_blocks: Vec<(Box<Expression>, Vec<Statement>)>,
        otherwise: Option<Vec<Statement>>
    ) -> Self { 
        Self { 
            cond_blocks,
            otherwise
        } 
    }

    pub fn get_conditional_blocks(&self) -> 
        &Vec<(Box<Expression>, Vec<Statement>)> { &self.cond_blocks }
    
    pub fn get_otherwise(&self) -> &Option<Vec<Statement>> { &self.otherwise }

}

impl Parse for If {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        let mut cond_blocks = Vec::new();
        let mut otherwise = None;

        lexer.next();
        let exp = Expression::parse(lexer)?;
        let tok = lexer.peek().clone();
        Self::expect_peek(lexer, TokenType::OpenCurlyBracket, ParseError::ExpectedOpenCurlyBracket(tok))?;
        let stmt = Statement::parse_block(lexer)?;
        cond_blocks.push((Box::new(exp),stmt));

        while lexer.peek().token_type == TokenType::Else {
            lexer.next();
            let tok = lexer.next();
            
            match tok.token_type {
                TokenType::If => {
                    lexer.next();
                    let exp = Expression::parse(lexer)?;
                    let tok = lexer.peek().clone();
                    Self::expect_peek(lexer, TokenType::OpenCurlyBracket, ParseError::ExpectedOpenCurlyBracket(tok))?;
                    let stmt = Statement::parse_block(lexer)?;
                    cond_blocks.push((Box::new(exp),stmt));
                },
                TokenType::OpenCurlyBracket => {
                    let stmt = Statement::parse_block(lexer)?;
                    otherwise = Some(stmt)
                }
                _ => return Err(ParseError::ExpectedSemicolon(tok.clone()))
            }
        }

        Ok(If::new(cond_blocks, otherwise))
    }
}