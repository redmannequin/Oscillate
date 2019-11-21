use crate::Parse;
use crate::Lexer;

use crate::Result;
use crate::error::ParseError;

use crate::TokenType;

//
// Expressions 
//

mod assign;
pub use assign::Assign;

mod identifier;
pub use identifier::Identifier;

mod real;
pub use real::Real;

#[derive(Debug)]
pub enum Expression {
    Assign(Assign),
    Identifier(Identifier),
    Real(Real)
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
}

impl Parse for Expression {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        let tok = lexer.curr().token_type.clone();
        match tok {

            TokenType::Identifier(_) => match lexer.peek().token_type.clone() {
                TokenType::Colon => Self::parse_assign(lexer),
                _ => Self::parse_identifier(lexer)
            }

            TokenType::Number(_) => Self::parse_real(lexer),

            _ => Err(ParseError::InvalidSoruce)
        }
    }
}