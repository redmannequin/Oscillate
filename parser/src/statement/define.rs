use crate::Lexer;
use crate::TokenType;

use crate::Result;
use crate::error::ParseError;

use crate::parse::Parse;

use crate::expression::Expression;
use crate::expression::Identifier;

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

impl Parse for Define {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        lexer.next();
        let name = Identifier::parse(lexer)?;
        let tok = lexer.peek().clone();
        Self::expect_peek(lexer, TokenType::OpenCurlyBracket, ParseError::ExpectedOpenCurlyBracket(tok))?;

        let mut body = Vec::new();
        loop {
            let tok = lexer.next().token_type.clone();
            if TokenType::CloseCurlyBracket == tok { break; }
            body.push(Expression::parse(lexer)?)
        }

        Ok(Define::new(name, body))
    }
}