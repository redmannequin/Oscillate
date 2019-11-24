use crate::TokenType;
use crate::Lexer;

use crate::parser::expect_peek;

use crate::Result;
use crate::error::ParseError;

use crate::traits::ParseTrait;
use crate::traits::LexerTrait;

use crate::expression::Expression;
use crate::expression::Identifier;

/// Define
/// 
#[derive(Debug, Clone)]
pub struct Define {
    name: Identifier,
    body: Vec<Expression>
}

impl Define {
    pub fn new(name: Identifier, body: Vec<Expression>) -> Self {
        Self { name, body }
    }

    pub fn get_name(&self) -> &Identifier { &self.name }

    pub fn get_body(&self) -> &Vec<Expression> { &self.body }
}

impl ParseTrait for Define {
    type Lexer = Lexer;

    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        lexer.next();
        let name = Identifier::parse(lexer)?;
        let tok = lexer.peek().clone();
        expect_peek(lexer, TokenType::OpenCurlyBracket, ParseError::ExpectedOpenCurlyBracket(tok))?;

        let mut body = Vec::new();
        loop {
            let tok = lexer.next().token_type.clone();
            if TokenType::CloseCurlyBracket == tok { break; }
            body.push(Expression::parse(lexer)?)
        }

        Ok(Define::new(name, body))
    }
}