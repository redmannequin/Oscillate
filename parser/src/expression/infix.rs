use crate::Lexer;
use crate::Result;
use crate::parse::Parse;
use crate::error::ParseError;
use crate::Token;
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
        let tok = lexer.curr();
        let infix = match tok.token_type {
            TokenType::Equal => (Some(InfixEnum::Equal), Precedence::Equals),
            TokenType::NotEqual => (Some(InfixEnum::NotEqual), Precedence::Equals),
            TokenType::LessThan => (Some(InfixEnum::LessThan), Precedence::LessGreater),
            TokenType::GraterThan => (Some(InfixEnum::GraterThan), Precedence::LessGreater),
            TokenType::Plus => (Some(InfixEnum::Plus), Precedence::Sum),
            TokenType::Minus => (Some(InfixEnum::Minus), Precedence::Sum),
            TokenType::Star => (Some(InfixEnum::Star), Precedence::Product),
            TokenType::Divide => (Some(InfixEnum::Divide), Precedence::Product),
            _ => return Err(ParseError::ExpectedInfix(tok.clone()))
        };
        Ok(infix)
    }
}

#[derive(Debug, Clone)]
pub struct Infix {
    pub infix: InfixType,
    pub left_exp: Box<Expression>,
    pub right_exp: Box<Expression>
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

    pub fn precedence(tok: &Token) -> Result<Precedence> {
        Ok(match tok.token_type {
            TokenType::Equal => Precedence::Equals,
            TokenType::NotEqual => Precedence::Equals,
            TokenType::LessThan => Precedence::LessGreater,
            TokenType::GraterThan => Precedence::LessGreater,
            TokenType::Plus => Precedence::Sum,
            TokenType::Minus => Precedence::Sum,
            TokenType::Star => Precedence::Product,
            TokenType::Divide => Precedence::Product,
            _ => return Err(ParseError::ExpectedInfix(tok.clone()))
        })
    }
}