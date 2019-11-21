
mod define;
pub use define::Define;

mod set_stmt;
pub use set_stmt::Set;

mod use_stmt;
pub use use_stmt::Use;

use crate::expression::Expression;

use crate::Parse;
use crate::Lexer;
use crate::Result;
use crate::TokenType;

#[derive(Debug)]
pub enum Statement {
    Define(Define),
    Set(Set),
    Use(Use),

    Expression(Expression),
}

impl Statement {

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

//
//  PARSE
//

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
