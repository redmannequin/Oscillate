use crate::Lexer;
use crate::TokenType;

use crate::parser::expect_peek;

use crate::traits::LexerTrait;
use crate::traits::ParseTrait;

use crate::Result;
use crate::error::ParseError;

use crate::expression::Expression;
use crate::statement::Statement;

/// If 
/// 
#[derive(Debug, Clone)]
pub struct If {
    pub cond_blocks: Vec<(Box<Expression>, Vec<Statement>)>,
    pub otherwise: Option<Vec<Statement>>
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
}

impl ParseTrait for If {
    type Lexer = Lexer;

    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        let mut cond_blocks = Vec::new();
        let mut otherwise = None;

        lexer.next();
        let exp = Expression::parse(lexer)?;
        let tok = lexer.peek().clone();
        expect_peek(lexer, TokenType::OpenCurlyBracket, ParseError::ExpectedOpenCurlyBracket(tok))?;
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
                    expect_peek(lexer, TokenType::OpenCurlyBracket, ParseError::ExpectedOpenCurlyBracket(tok))?;
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