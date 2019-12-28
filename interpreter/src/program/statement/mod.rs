
mod block;
pub use block::Block;

mod define;
pub use define::Mod;

mod set_stmt;
pub use set_stmt::Set;

mod use_stmt;
pub use use_stmt::Use;

use crate::Lexer;
use crate::TokenType;
use crate::Object;
use crate::Container;

use crate::parser::expect_curr;

use crate::traits::ParseTrait;
use crate::traits::EvalTrait;
use crate::traits::LexerTrait;
use crate::traits::NamespaceTrait;

use crate::Result;
use crate::error::ParseError;

use crate::program::expression::Expression;

/// Statemnet
/// 
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Mod(Mod),
    Set(Set),
    Use(Use),
    Expression(Expression),
}

impl Statement {

    pub fn parse_block(lexer: &mut Lexer) -> Result<Vec<Self>> {
        let mut block = Vec::new();
        let tok = lexer.curr().clone();
        expect_curr(lexer, TokenType::OpenCurlyBracket, ParseError::ExpectedOpenCurlyBracket(tok))?;
        loop {
            let tok = lexer.next().token_type.clone();
            let stmt = match tok {
                TokenType::Use => Self::parse_use(lexer)?,
                TokenType::Set => Self::parse_set(lexer)?,
                TokenType::Mod => Self::parse_mod(lexer)?,
                TokenType::CloseCurlyBracket => break,
                _ => Self::parse_expression(lexer)?
            };
            block.push(stmt);
        }
        Ok(block)
    }

    fn parse_mod(lexer: &mut Lexer) -> Result<Self> {
        let define = Mod::parse(lexer)?;
        let stmt = Statement::Mod(define);
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
impl ParseTrait for Statement {
    type Lexer = Lexer;

    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        let tok = lexer.curr().token_type.clone();
        match tok {
            TokenType::Use => Self::parse_use(lexer),
            TokenType::Set => Self::parse_set(lexer),
            TokenType::Mod => Self::parse_mod(lexer),
            _ => Self::parse_expression(lexer)
        }
    }
}

/// Statement Eval
///  
impl EvalTrait<Object> for Statement {

    fn eval(&self, env: Container<impl NamespaceTrait<Object>>) -> Result<Object> {
        match self {
            Statement::Mod(_) => Err(ParseError::Ops),
            Statement::Set(_) => Err(ParseError::Ops),
            Statement::Use(_) => Err(ParseError::Ops),
            Statement::Expression(exp) => exp.eval(env)
        }
    }
}

// #[test]
// fn statement() {
//     unimplemented!()
// }
