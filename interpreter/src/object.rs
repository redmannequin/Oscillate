/// Object
/// 
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
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
