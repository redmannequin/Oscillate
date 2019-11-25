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

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod infix_tests {
    use crate::Object;
    use crate::Container;
    use crate::Env;
    use crate::Lexer;

    use crate::traits::LexerTrait;
    use crate::traits::ParseTrait;
    use crate::traits::EvalTrait;

    use crate::expression::Expression;

    use super::Infix;
    use super::InfixType;
    use super::InfixEnum;
    use super::Precedence;

    #[test]
    fn infix_type() {
        let source = "+";
        let mut lexer = Lexer::new(String::from(source));
        lexer.next();

        let infix_type = InfixType::parse(&mut lexer);
        assert!(infix_type.is_ok(), "Prefix parse failed: {:?}", infix_type);
        let infix_type = infix_type.unwrap();

        assert_eq!(infix_type, (Some(InfixEnum::Plus), Precedence::Sum));
    }

    #[test]
    fn equal_infix() {
        use crate::expression::Real;
        let env = Container::new(Env::new());

        let infix = (Some(InfixEnum::Equal), Precedence::Equals);
        let left_exp = Expression::Real(Real::new(0.5));
        let right_exp = Expression::Real(Real::new(0.5));

        let infix = Infix::new(infix, left_exp, right_exp);
        let obj = infix.eval(env);
        assert!(obj.is_ok(), "Infix eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Bool(true));
    }

    #[test]
    fn not_equal_infix() {
        use crate::expression::Real;
        let env = Container::new(Env::new());

        let infix = (Some(InfixEnum::NotEqual), Precedence::Equals);
        let left_exp = Expression::Real(Real::new(0.5));
        let right_exp = Expression::Real(Real::new(0.5));

        let infix = Infix::new(infix, left_exp, right_exp);
        let obj = infix.eval(env);
        assert!(obj.is_ok(), "Infix eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Bool(false));
    }

    #[test]
    fn less_than_infix() {
        use crate::expression::Real;
        let env = Container::new(Env::new());

        let infix = (Some(InfixEnum::LessThan), Precedence::LessGreater);
        let left_exp = Expression::Real(Real::new(0.5));
        let right_exp = Expression::Real(Real::new(0.5));

        let infix = Infix::new(infix, left_exp, right_exp);
        let obj = infix.eval(env);
        assert!(obj.is_ok(), "Infix eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Bool(false));
    }

    #[test]
    fn grater_than_infix() {
        use crate::expression::Real;
        let env = Container::new(Env::new());

        let infix = (Some(InfixEnum::GraterThan), Precedence::LessGreater);
        let left_exp = Expression::Real(Real::new(0.5));
        let right_exp = Expression::Real(Real::new(0.5));

        let infix = Infix::new(infix, left_exp, right_exp);
        let obj = infix.eval(env);
        assert!(obj.is_ok(), "Infix eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Bool(false));
    }

    #[test]
    fn plus_infix() {
        use crate::expression::Real;
        let env = Container::new(Env::new());

        let infix = (Some(InfixEnum::Plus), Precedence::Sum);
        let left_exp = Expression::Real(Real::new(0.5));
        let right_exp = Expression::Real(Real::new(0.5));

        let infix = Infix::new(infix, left_exp, right_exp);
        let obj = infix.eval(env);
        assert!(obj.is_ok(), "Infix eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Real(1.0));
    }

    #[test]
    fn minus_infix() {
        use crate::expression::Real;
        let env = Container::new(Env::new());

        let infix = (Some(InfixEnum::Minus), Precedence::Sum);
        let left_exp = Expression::Real(Real::new(0.5));
        let right_exp = Expression::Real(Real::new(0.5));

        let infix = Infix::new(infix, left_exp, right_exp);
        let obj = infix.eval(env);
        assert!(obj.is_ok(), "Infix eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Real(0.0));
    }

    #[test]
    fn star_infix() {
        use crate::expression::Real;
        let env = Container::new(Env::new());

        let infix = (Some(InfixEnum::Star), Precedence::Product);
        let left_exp = Expression::Real(Real::new(0.5));
        let right_exp = Expression::Real(Real::new(0.5));

        let infix = Infix::new(infix, left_exp, right_exp);
        let obj = infix.eval(env);
        assert!(obj.is_ok(), "Infix eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Real(0.25));
    }

    #[test]
    fn divide_infix() {
        use crate::expression::Real;
        let env = Container::new(Env::new());

        let infix = (Some(InfixEnum::Divide), Precedence::Product);
        let left_exp = Expression::Real(Real::new(0.5));
        let right_exp = Expression::Real(Real::new(0.5));

        let infix = Infix::new(infix, left_exp, right_exp);
        let obj = infix.eval(env);
        assert!(obj.is_ok(), "Infix eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Real(1.0));
    }
}