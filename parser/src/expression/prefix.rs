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
    prefix: PrefixType,
    expression: Box<Expression>
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
        let tok = &lexer.curr().token_type;
        let prefix_type = match tok {
            TokenType::Minus => PrefixType::Minus,
            TokenType::Not => PrefixType::Not,
            _ => return Err(ParseError::ExpectedPrefix)
        };
        lexer.next();
        let exp = Expression::parse(lexer)?;
        Ok(Prefix::new(prefix_type, exp))
    }
}