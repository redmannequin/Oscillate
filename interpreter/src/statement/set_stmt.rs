use crate::Lexer;
use crate::Result;

use crate::traits::ParseTrait;

#[derive(Debug, Clone)]
pub struct Set {}

impl Set {
    pub fn new() -> Self { Self {} }
}

impl ParseTrait for Set {
    type Lexer = Lexer;

    fn parse(_lexer: &mut Self::Lexer) -> Result<Self> {
        Ok(Set::new())
    }
}