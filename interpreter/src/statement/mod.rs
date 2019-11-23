
mod define;
pub use define::Define;

mod set_stmt;
pub use set_stmt::Set;

mod use_stmt;
pub use use_stmt::Use;

use crate::Lexer;
use crate::TokenType;
use crate::Object;

use crate::traits::Parse;
use crate::traits::Eval;
use crate::traits::Environment;
use crate::traits::Container;

use crate::Result;
use crate::error::ParseError;

use crate::expression::Expression;

/// Statemnet
/// 
#[derive(Debug, Clone)]
pub enum Statement {
    Define(Define),
    Set(Set),
    Use(Use),
    Expression(Expression),
}

impl Statement {

    pub fn parse_block(lexer: &mut Lexer) -> Result<Vec<Self>> {
        let mut block = Vec::new();
        let tok = lexer.curr().clone();
        Self::expect_curr(lexer, TokenType::OpenCurlyBracket, ParseError::ExpectedOpenCurlyBracket(tok))?;
        loop {
            let tok = lexer.next().token_type.clone();
            let stmt = match tok {
                TokenType::Use => Self::parse_use(lexer)?,
                TokenType::Set => Self::parse_set(lexer)?,
                TokenType::Define => Self::parse_define(lexer)?,
                TokenType::CloseCurlyBracket => break,
                _ => Self::parse_expression(lexer)?
            };
            block.push(stmt);
        }
        Ok(block)
    }

    fn parse_define(lexer: &mut Lexer) -> Result<Self> {
        let define = Define::parse(lexer)?;
        let stmt = Statement::Define(define);
        Ok(stmt)
    }

    fn parse_set(lexer: &mut Lexer) -> Result<Self> {
        let set_stmt = Set::parse(lexer)?;
        let stmt = Statement::Set(set_stmt);
        Ok(stmt)
    }

    fn parse_use(lexer: &mut Lexer) -> Result<Self> {
        let use_stmt = Use::parse(lexer)?;
        let stmt = Statement::Use(use_stmt);
        Ok(stmt)
    }

    fn parse_expression(lexer: &mut Lexer) ->  Result<Self> {
        let exp = Expression::parse(lexer)?;
        let stmt = Statement::Expression(exp);
        Ok(stmt)
    }

}

/// Statment Parse
///
impl Parse for Statement {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        let tok = lexer.curr().token_type.clone();
        match tok {
            TokenType::Use => Self::parse_use(lexer),
            TokenType::Set => Self::parse_set(lexer),
            TokenType::Define => Self::parse_define(lexer),
            _ => Self::parse_expression(lexer)
        }
    }
}

/// Statement Eval
///  
impl Eval for Statement {
    fn eval(&self, env: Container<impl Environment>) -> Result<Object> {
        match self {
            Statement::Define(_) => Err(ParseError::Ops),
            Statement::Set(_) => Err(ParseError::Ops),
            Statement::Use(_) => Err(ParseError::Ops),
            Statement::Expression(exp) => exp.eval(env)
        }
    }
}