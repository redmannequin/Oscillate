use crate::Lexer;
use crate::TokenType;

use crate::traits::LexerTrait;
use crate::traits::ParseTrait;

use crate::Result;
use crate::error::ParseError;


/// Bool
/// 
#[derive(Debug, Clone)]
pub struct Bool {
    pub value: bool
}

impl Bool {
    pub fn new(value: bool) -> Self { Self { value } }
}

impl ParseTrait for Bool {
    type Lexer = Lexer;
    
    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        let tok = lexer.curr();
        match tok.token_type {
            TokenType::True => Ok(Bool::new(true)),
            TokenType::False => Ok(Bool::new(false)),
            _ => Err(ParseError::ExpectedBool(tok.clone()))
        }
    }
}

#[test]
fn bool() {
    unimplemented!()
}