//! The suite of traits to abstract over the interpreters Environment, Eval, Parse and 
//! Token functionalities, making it easy to alter and extend the interpreter with ease.

use crate::Container;
use crate::Result;

/// A Namespace provides a key-value store for variables, functions, modules, and so on.
pub trait NamespaceTrait<O> {

    /// Returns the value of an identifier if it exists
    fn get(&self, name: &str) -> Option<Container<O>>;
    
    /// sets and adds namespace key-value pairs
    fn set(&mut self, name: &str, obj: O);
}


/// An Eval implementer creates an Object from itself
/// 
/// Please note that that the `Namespace` used must share the `Object` type generated by `Eval`
pub trait EvalTrait: Sized {
    /// The type used to represent evaluated results
    type Object;
    /// The `Namespace` used by the evaluator
    type Namespace: NamespaceTrait<Self::Object>;
    

    /// Returns an evaluated object
    /// 
    fn eval(&self, env: Container<Self::Namespace>) -> Result<Self::Object>;
}


/// A Parse implementer created itself from a stream of tokens
pub trait ParseTrait: Sized {
    /// the lexer type used 
    type Lexer: LexerTrait;

    /// Returns an instance of the implementer
    /// 
    fn parse(lexer: &mut Self::Lexer) -> Result<Self>;
}

/// A Lexer implementer generates tokens
pub trait LexerTrait {
    /// The token type the lexer will generate 
    type Token: TokenTrait;

    /// generates the next token
    /// 
    fn next(&mut self) -> &Self::Token;
    
    /// lets you peek the next token
    /// 
    fn peek(&self) -> &Self::Token;
    
    /// gets the current token
    /// 
    fn curr(&self) -> &Self::Token;
}

/// A Token implementer ....
pub trait TokenTrait {
    /// The type used to represent token types
    type TokenType: PartialEq;

    /// get the `TokenType`
    /// 
    fn get_type(&self) -> &Self::TokenType;
    
    /// get the line the token was generated
    /// 
    fn get_line(&self) -> u32;
}