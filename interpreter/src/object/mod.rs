
mod env;
pub use env::Env;

use crate::Container;


/// Object
/// 
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Func(Vec<String>, Container<Env<Object>>),
    Array(Vec<Object>),
    Bool(bool),
    Real(f64),
    Null
}

impl Object {

    pub fn is_true(&self) -> bool {
        match self {
            Object::Bool(value) => *value,
            Object::Null => false,
            _ => true,
        }
    }
    
}

impl From<f64> for Object {
    fn from(item: f64) -> Self {
        Object::Real(item)
    }
}