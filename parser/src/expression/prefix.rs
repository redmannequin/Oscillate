use crate::Lexer;
use crate::Result;
use crate::parse::Parse;
use crate::error::ParseError;
use crate::TokenType;

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