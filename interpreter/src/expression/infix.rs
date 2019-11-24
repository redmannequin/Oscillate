use crate::Lexer;
use crate::Token;
use crate::TokenType;
use crate::Object;
use crate::Container;
use crate::Env;

use crate::traits::LexerTrait;
use crate::traits::ParseTrait;
use crate::traits::EvalTrait;

use crate::Result;
use crate::error::ParseError;

use crate::expression::Expression;

/// Precedence
/// 
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

impl ParseTrait for InfixType {
    type Lexer = Lexer;

    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
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

    fn eval_integer_infix_exp(&self, left: f64, right: f64) -> Result<Object> {
        Ok(match self.infix.0 {
            Some(InfixEnum::Equal) => Object::Bool(left == right),
            Some(InfixEnum::NotEqual) => Object::Bool(left != right),
            Some(InfixEnum::LessThan) => Object::Bool(left < right),
            Some(InfixEnum::GraterThan) => Object::Bool(left > right),
            Some(InfixEnum::Plus) => Object::Real(left + right),
            Some(InfixEnum::Minus) => Object::Real(left - right),
            Some(InfixEnum::Star) => Object::Real(left * right),
            Some(InfixEnum::Divide) => Object::Real(left / right),
            None => return Err(ParseError::Ops)
        })
    }
}

impl EvalTrait for Infix {
    type Object = Object;
    type Namespace = Env<Object>;

    fn eval(&self, env: Container<Self::Namespace>) -> Result<Object> {
        let left_obj = Expression::eval(self.left_exp.as_ref(), env.clone())?;
        let right_obj = Expression::eval(self.right_exp.as_ref(), env.clone())?;

        match (left_obj, right_obj) {
            (Object::Real(left), Object::Real(right)) => {
                self.eval_integer_infix_exp(left, right)
            },
            _ => Err(ParseError::Ops)
        }
    }
}