//! # How the Interperter works 
//! Magic

mod container;
pub use container::Container;

pub mod error;
pub use error::Result;

mod evaluator;
pub use evaluator::Evaluator;

mod lexer;
pub use lexer::Lexer;

mod object;
pub use object::Object;
pub use object::Env;

pub mod parser;

mod program;
pub use program::Program;

mod token;
pub use token::Token;
pub use token::TokenType;

pub mod traits;