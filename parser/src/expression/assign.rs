use crate::Lexer;
use crate::Object;

use crate::traits::Parse;
use crate::traits::Eval;
use crate::traits::Environment;
use crate::traits::Container;

use crate::expression::Identifier;
use crate::expression::Expression;

use crate::Result;

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

impl Parse for Assign {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        // get name
        let ident = Identifier::parse(lexer)?;
        
        // read colon
        lexer.next();
        lexer.next();

        // get exp
        let value = Expression::parse(lexer)?;
        
        // check for semicolon and read
        Self::on_semicolon(lexer)?;

        Ok(Assign::new(ident,value))
    }
}

impl Eval for Assign {
    fn eval(&self, env: Container<impl Environment>) -> Result<Object> {
        let result = self.value.eval(env.clone())?;
        env.get_mut().set(self.ident.name.as_ref(), result.clone());
        Ok(result)
    }
}