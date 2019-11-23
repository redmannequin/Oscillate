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

#[derive(Debug, Clone)]
pub enum PrefixType {
    Minus,
    Not,
}

#[derive(Debug, Clone)]
pub struct Prefix {
    pub prefix: PrefixType,
    pub expression: Box<Expression>
}

impl Prefix {
    pub fn new(prefix: PrefixType, expression: Expression) -> Self {
        let expression = Box::new(expression);
        Self {
            prefix,
            expression
        }
    }
}

impl Parse for Prefix {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        let tok = lexer.curr();
        let prefix_type = match tok.token_type {
            TokenType::Minus => PrefixType::Minus,
            TokenType::Not => PrefixType::Not,
            _ => return Err(ParseError::ExpectedPrefix(tok.clone()))
        };
        lexer.next();
        let exp = Expression::parse(lexer)?;
        Ok(Prefix::new(prefix_type, exp))
    }
}

impl Eval for Prefix {
    fn eval(&self, env: Container<impl Environment>) -> Result<Object> {
        let obj = Expression::eval(self.expression.as_ref(), env)?;
        match self.prefix {
            PrefixType::Not => Err(ParseError::Ops),
            PrefixType::Minus => match obj {
                Object::Real(num) => Ok(Object::Real(-num)),
                _ =>  Err(ParseError::Ops)
            } 
        }
    }
}