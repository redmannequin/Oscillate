use crate::Lexer;
use crate::Result;
use crate::parse::Parse;

use crate::expression::Identifier;
use crate::expression::Expression;

#[derive(Debug, Clone)]
pub struct Assign {
    name: Identifier,
    value: Box<Expression>
}

impl Assign {
    pub fn new(name: Identifier, value: Expression) -> Self {
        let value = Box::new(value); 
        Self { name, value }
    }

    pub fn get_name(&self) -> &Identifier { &self.name }
    pub fn get_value(&self) -> &Box<Expression> { &self.value }
}

impl Parse for Assign {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        // get name
        let name = Identifier::parse(lexer)?;
        
        // read colon
        lexer.next();
        lexer.next();

        // get exp
        let value = Expression::parse(lexer)?;
        println!("{:?}",lexer.curr());
        
        // check for semicolon and read
        Self::on_semicolon(lexer)?;

        Ok(Assign::new(name,value))
    }
}