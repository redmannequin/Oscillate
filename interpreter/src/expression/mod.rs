use crate::Lexer;
use crate::Object;
use crate::TokenType;
use crate::Container;
use crate::Env;

use crate::traits::LexerTrait;
use crate::traits::ParseTrait;
use crate::traits::EvalTrait;

use crate::Result;
use crate::error::ParseError;

mod assign;
pub use assign::Assign;

mod bool_exp;
pub use bool_exp::Bool;

mod identifier;
pub use identifier::Identifier;

mod if_exp;
pub use if_exp::If;

mod infix;
pub use infix::Infix;
pub use infix::InfixType;
pub use infix::InfixEnum;
pub use infix::Precedence;

mod prefix;
pub use prefix::Prefix;
pub use prefix::PrefixType;

mod real;
pub use real::Real;

/// Expression
/// 
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Assign(Assign),
    Bool(Bool),
    Identifier(Identifier),
    If(If),
    Infix(Infix),
    Prefix(Prefix),
    Real(Real),
    Null
}

impl Expression {

    fn parse_identifier(lexer: &mut Lexer) -> Result<Self> {
        let ident = Identifier::parse(lexer)?;
        let exp = Expression::Identifier(ident);
        Ok(exp)
    } 

    fn parse_real(lexer: &mut Lexer) -> Result<Self> {
        let real = Real::parse(lexer)?;
        let exp = Expression::Real(real);
        Ok(exp)
    }

    fn parse_assign(lexer: &mut Lexer) -> Result<Self> {
        let assign = Assign::parse(lexer)?;
        let exp = Expression::Assign(assign);
        Ok(exp)
    }

    fn parse_bool(lexer: &mut Lexer) -> Result<Self> {
        let bool_lit = Bool::parse(lexer)?;
        let exp = Expression::Bool(bool_lit);
        Ok(exp)
    }

    fn parse_if(lexer: &mut Lexer) -> Result<Self> {
        let if_e = If::parse(lexer)?;
        let exp = Expression::If(if_e);
        Ok(exp)
    }

    fn parse_prefix(lexer: &mut Lexer) -> Result<Self> {
        let prefix = Prefix::parse(lexer)?;
        let exp = Expression::Prefix(prefix);
        Ok(exp)
    }

    fn parse_left_exp(lexer: &mut Lexer) -> Result<Self> {
        let tok = lexer.curr();
        let left_exp = match tok.token_type {
            TokenType::Identifier(_) => match lexer.peek().token_type.clone() {
                TokenType::Colon => Self::parse_assign(lexer)?,
                _ => Self::parse_identifier(lexer)?
            }
            TokenType::Number(_) => Self::parse_real(lexer)?,
            TokenType::True => Self::parse_bool(lexer)?,
            TokenType::False => Self::parse_bool(lexer)?,
            TokenType::If => Self::parse_if(lexer)?,
            TokenType::Not => Self::parse_prefix(lexer)?,
            TokenType::Minus => Self::parse_prefix(lexer)?,
            _ => return Err(ParseError::ExpectedExpression(tok.clone()))
        };
        Ok(left_exp)
    }

    fn parse_exp(lexer: &mut Lexer, precedence: Precedence) -> Result<Self> {
        let mut left_exp = Self::parse_left_exp(lexer)?;

        loop {
            
            let tok = lexer.peek();
            match Infix::precedence(tok) {
                Ok(prec) => if precedence > prec { break; },
                Err(_) => break
            }
            lexer.next();
            let infix = InfixType::parse(lexer)?;
            lexer.next();
            let right_exp = Self::parse_exp(lexer, infix.1)?;
            left_exp = Expression::Infix(Infix::new(infix, left_exp, right_exp));
        }

        let tok = lexer.peek();
        if tok.token_type == TokenType::Semicolon {
            lexer.next();
        }

        Ok(left_exp)
    }

}

impl ParseTrait for Expression {
    type Lexer = Lexer;

    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        Self::parse_exp(lexer, Precedence::Lowest)
    }
}

impl EvalTrait for Expression {
    type Object = Object;
    type Namespace = Env<Object>;

    fn eval(&self, env: Container<Self::Namespace>) -> Result<Object> {
        match self {
            Expression::Assign(assign) => assign.eval(env),
            Expression::Real(num) => num.eval(env),
            Expression::Prefix(prefix) => prefix.eval(env),
            Expression::Identifier(ident) => ident.eval(env),
            Expression::Infix(infix) => infix.eval(env),
            Expression::Bool(bool_exp) => bool_exp.eval(env),
            Expression::If(if_exp) => if_exp.eval(env),
            Expression::Null => Ok(Object::Null)
        }
    }
}

// #[cfg(test)]
// mod expression_tests {

//     #[test]
//     fn left_expression() {
//         unimplemented!()
//     }

//     #[test]
//     fn expression() {
//         unimplemented!()
//     }

// }