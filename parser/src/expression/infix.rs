use crate::Lexer;
use crate::Result;
use crate::parse::Parse;
use crate::error::ParseError;
use crate::TokenType;

use crate::expression::Expression;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
}

#[derive(Debug, Clone, Copy)]
pub enum InfixEnum {
    Equal,
    NotEqual,
    LessThan,
    GraterThan,
    Plus,
    Minus,
    Star,
    Divide,
}

pub type InfixType = (Option<InfixEnum>, Precedence);

impl Parse for InfixType {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        let tok = &lexer.curr().token_type;
        let infix = match tok {
            TokenType::Equal => (Some(InfixEnum::Equal), Precedence::Equals),
            TokenType::NotEqual => (Some(InfixEnum::NotEqual), Precedence::Equals),
            TokenType::LessThan => (Some(InfixEnum::LessThan), Precedence::LessGreater),
            TokenType::GraterThan => (Some(InfixEnum::GraterThan), Precedence::LessGreater),
            TokenType::Plus => (Some(InfixEnum::Plus), Precedence::Sum),
            TokenType::Minus => (Some(InfixEnum::Minus), Precedence::Sum),
            TokenType::Star => (Some(InfixEnum::Star), Precedence::Product),
            TokenType::Divide => (Some(InfixEnum::Divide), Precedence::Product),
            _ => return Err(ParseError::ExpectedInfix)
        };
        Ok(infix)
    }
}

#[derive(Debug, Clone)]
pub struct Infix {
    infix: InfixType,
    left_exp: Box<Expression>,
    right_exp: Box<Expression>
}

impl Infix {
    pub fn new(infix: InfixType, left_exp: Expression, right_exp: Expression) -> Self {
        let left_exp = Box::new(left_exp);
        let right_exp = Box::new(right_exp);
        Self {
            infix,
            left_exp,
            right_exp
        }
    }
}
