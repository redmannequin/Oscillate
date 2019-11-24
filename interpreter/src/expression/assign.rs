use crate::Lexer;
use crate::Object;
use crate::Container;
use crate::Env;

use crate::parser::on_semicolon;

use crate::traits::LexerTrait;
use crate::traits::ParseTrait;
use crate::traits::EvalTrait;
use crate::traits::NamespaceTrait;

use crate::expression::Identifier;
use crate::expression::Expression;

use crate::Result;

/// Assign
/// 
#[derive(Debug, Clone)]
pub struct Assign {
    pub ident: Identifier,
    pub value: Box<Expression>
}

impl Assign {
    pub fn new(ident: Identifier, value: Expression) -> Self {
        let value = Box::new(value); 
        Self { ident, value }
    }
}

impl ParseTrait for Assign {
    type Lexer = Lexer;

    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        // get name
        let ident = Identifier::parse(lexer)?;
        
        // read colon
        lexer.next();
        lexer.next();

        // get exp
        let value = Expression::parse(lexer)?;
        
        // check for semicolon and read
        on_semicolon(lexer)?;

        Ok(Assign::new(ident,value))
    }
}

impl EvalTrait for Assign {
    type Object = Object;
    type Namespace = Env<Object>;

    fn eval(&self, env: Container<Self::Namespace>) -> Result<Object> {
        let result = self.value.eval(env.clone())?;
        env.get_mut().set(self.ident.name.as_ref(), result.clone());
        Ok(result)
    }
}