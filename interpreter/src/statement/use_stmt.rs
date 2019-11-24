use crate::Lexer;
use crate::Result;

use crate::traits::ParseTrait;

#[derive(Debug, Clone)]
pub struct Use {}

impl Use {
    pub fn new() -> Self { Self {} }
}

impl ParseTrait for Use {
    type Lexer = Lexer;

    fn parse(_lexer: &mut Self::Lexer) -> Result<Self> {
        Ok(Use::new())
    }
}