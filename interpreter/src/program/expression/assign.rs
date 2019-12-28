use crate::Lexer;
use crate::Object;
use crate::Container;

use crate::parser::on_semicolon;

use crate::traits::LexerTrait;
use crate::traits::ParseTrait;
use crate::traits::EvalTrait;
use crate::traits::NamespaceTrait;

use crate::program::expression::Identifier;
use crate::program::expression::Expression;

use crate::Result;

/// Assign
/// 
#[derive(Debug, Clone, PartialEq)]
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

impl EvalTrait<Object> for Assign {
    
    fn eval(&self, env: Container<impl NamespaceTrait<Object>>) -> Result<Object> {
        let result = self.value.eval(env.clone())?;
        env.get_mut().set(self.ident.name.as_ref(), result.clone());
        Ok(result)
    }
}

#[cfg(test)]
mod assign_tests {
    use crate::Object;
    use crate::Container;
    use crate::Env;
    use crate::Lexer;

    use crate::traits::LexerTrait;
    use crate::traits::ParseTrait;
    use crate::traits::EvalTrait;

    use crate::program::expression::Real;
    use crate::program::expression::Identifier;
    use crate::program::expression::Expression;

    use super::Assign;

    #[test]
    fn assign() {
        let source = "a:5;";
        let env = Container::new(Env::new());

        let mut lexer = Lexer::new(String::from(source));
        lexer.next();

        let assign = Assign::parse(&mut lexer);
        assert!(assign.is_ok(), "Assign parse failed: {:?}", assign);
        let assign = assign.unwrap();

        let ast = Assign::new(
            Identifier::new(String::from("a")), 
            Expression::Real(Real::new(5.0))
        );

        assert_eq!(assign, ast);
        
        let obj = assign.eval(env);
        assert!(obj.is_ok(), "Assign eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Real(5.0));
    }
}