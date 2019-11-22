use crate::Lexer;
use crate::Result;
use crate::parse::Parse;

use crate::expression::Identifier;
use crate::expression::Expression;

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

    pub fn get_ident(&self) -> &Identifier { &self.ident }
    pub fn get_value(&self) -> &Box<Expression> { &self.value }
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