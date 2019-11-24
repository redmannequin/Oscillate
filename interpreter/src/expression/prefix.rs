use crate::Lexer;
use crate::TokenType;
use crate::Object;
use crate::Container;
use crate::Env;

use crate::traits::LexerTrait;
use crate::traits::ParseTrait;
use crate::traits::EvalTrait;

use crate::Result;
use crate::error::ParseError;

use crate::expression::Expression;

/// PrefixType
/// 
#[derive(Debug, Clone)]
pub enum PrefixType {
    Minus,
    Not,
}

/// Prefix
/// 
#[derive(Debug, Clone)]
pub struct Prefix {
    pub prefix: PrefixType,
    pub expression: Box<Expression>
}

impl Prefix {
    pub fn new(prefix: PrefixType, expression: Expression) -> Self {
        let expression = Box::new(expression);
        Self {
            prefix,
            expression
        }
    }
}

impl ParseTrait for Prefix {
    type Lexer = Lexer;

    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        let tok = lexer.curr();
        let prefix_type = match tok.token_type {
            TokenType::Minus => PrefixType::Minus,
            TokenType::Not => PrefixType::Not,
            _ => return Err(ParseError::ExpectedPrefix(tok.clone()))
        };
        lexer.next();
        let exp = Expression::parse(lexer)?;
        Ok(Prefix::new(prefix_type, exp))
    }
}

impl EvalTrait for Prefix {
    type Object = Object;
    type Namespace = Env<Object>;

    fn eval(&self, env: Container<Self::Namespace>) -> Result<Object> {
        let obj = Expression::eval(self.expression.as_ref(), env)?;
        match self.prefix {
            PrefixType::Not => Err(ParseError::Ops),
            PrefixType::Minus => match obj {
                Object::Real(num) => Ok(Object::Real(-num)),
                _ =>  Err(ParseError::Ops)
            } 
        }
    }
}

#[test]
fn not_prefix() {
    let source = "!true;";
    let env = Container::new(Env::new());

    let mut lexer = Lexer::new(String::from(source));
    lexer.next();

    let prefix = Prefix::parse(&mut lexer);
    assert!(prefix.is_ok(), "Prefix parse failed: {:?}", prefix);
    let prefix = prefix.unwrap();
    
    let obj = prefix.eval(env);
    assert!(obj.is_ok(), "Prefix eval failed: {:?}", obj);
    let obj = obj.unwrap();

    assert_eq!(obj, Object::Bool(false));
}

#[test]
fn minus_prefix() {
    let source = "-5;";
    let env = Container::new(Env::new());

    let mut lexer = Lexer::new(String::from(source));
    lexer.next();

    let prefix = Prefix::parse(&mut lexer);
    assert!(prefix.is_ok(), "Prefix parse failed: {:?}", prefix);
    let prefix = prefix.unwrap();
    
    let obj = prefix.eval(env);
    assert!(obj.is_ok(), "Prefix eval failed: {:?}", obj);
    let obj = obj.unwrap();

    assert_eq!(obj, Object::Real(-5.0));
}