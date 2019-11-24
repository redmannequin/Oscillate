//! # How the Interperter works 
//! Magic

pub mod expression;
pub use expression::Expression;

pub mod statement;
pub use statement::Statement;

mod container;
pub use container::Container;

mod env;
pub use env::Env;

pub mod error;
pub use error::Result;

mod evaluator;
pub use evaluator::Evaluator;

mod lexer;
pub use lexer::Lexer;

mod object;
pub use object::Object;

mod parser;
pub use parser::Parser;

mod token;
pub use token::Token;
pub use token::TokenType;

pub mod traits;