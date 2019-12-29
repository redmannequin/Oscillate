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
pub use lexer::Token;
pub use lexer::TokenType;

mod object;
pub use object::Object;
pub use object::Env;

pub mod parser;

mod program;
pub use program::Program;


pub mod traits;