use crate::Lexer;
use crate::TokenType;
use crate::Object;
use crate::Container;

pub mod statement;
use statement::Statement;

pub mod expression;

use crate::traits::ParseTrait;
use crate::traits::EvalTrait;
use crate::traits::LexerTrait;
use crate::traits::NamespaceTrait;

use crate::Result;

pub struct Program {
    statements: Vec<Statement>
}

impl ParseTrait for Program {
    type Lexer = Lexer;

    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        let mut statements = Vec::new();
        loop {
            let tok = lexer.next().token_type.clone();
            let stmt = match tok {
                TokenType::EOF => break,
                TokenType::Illegal => break,
                _ => Statement::parse(lexer)?
            };
            statements.push(stmt);
        }
        Ok( Program { statements } )
    }

}

impl EvalTrait<Object> for Program {

    fn eval(&self, env: Container<impl NamespaceTrait<Object>>) -> Result<Object> {
        let mut result = Object::Null;
        for statement in &self.statements {
            result = statement.eval(env.clone())?;
        }
        Ok(result)
    }

}