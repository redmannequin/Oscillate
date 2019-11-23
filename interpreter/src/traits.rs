use crate::Lexer;
use crate::TokenType;
use crate::Object;

use crate::Result;
use crate::error::ParseError;

use std::rc::Rc;
use std::cell::Ref;
use std::cell::RefMut;
use std::cell::RefCell;

/// # Container
/// 
#[derive(Debug)]
pub struct Container<T>(Rc<RefCell<T>>);

impl<T> Container<T> {
    pub fn new(item: T) -> Self { Self(Rc::new(RefCell::new(item))) }
    pub fn get(&self) -> Ref<T> { self.0.borrow() }
    pub fn get_mut(&self) -> RefMut<T> { self.0.borrow_mut() }
    pub fn set(&self, item: T) { *self.0.borrow_mut() = item; }
}

impl<T> Clone for Container<T> {
    fn clone(&self) -> Self { Self(Rc::clone(&self.0)) }
}


/// # Environment Tarit
/// 
pub trait Environment {
    fn get(&self, name: &str) -> Option<Container<Object>>;
    fn set(&mut self, name: &str, obj: Object);
}


/// # Eval
/// 
pub trait Eval: Sized {
    fn eval(&self, env: Container<impl Environment>) -> Result<Object>;
}


/// # Parse
/// 
pub trait Parse: Sized {
    fn parse(lexer: &mut Lexer) -> Result<Self>;

    fn expect_curr(lexer: &mut Lexer, tok_type: TokenType, err: ParseError) -> Result<()> {
        let token = lexer.curr();
        if token.token_type == tok_type {
            Ok(())
        } else {
            Err(err)
        }
    }

    fn expect_peek(lexer: &mut Lexer, tok_type: TokenType, err: ParseError) -> Result<()> {
        let token = lexer.next();
        if token.token_type == tok_type {
            Ok(())
        } else {
            Err(err)
        }
    }

    fn expect_semicolon(lexer: &mut Lexer) -> Result<()> {
        let tok = lexer.next();
        if tok.token_type == TokenType::Semicolon {
            Ok(())
        } else {
            Err(ParseError::ExpectedSemicolon(tok.clone()))
        }
    }

    fn on_semicolon(lexer: &mut Lexer) -> Result<()> {
        let tok = lexer.curr();
        if tok.token_type == TokenType::Semicolon {
            Ok(())
        } else {
            Err(ParseError::ExpectedSemicolon(tok.clone()))
        }
    }
}