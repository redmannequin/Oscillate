use std::rc::Rc;
use std::cell::RefCell;

use crate::environment::Environment;
use parser::statement::Statement;

#[derive(Debug, Clone)]
pub enum Object {
    Bool(bool),
    Real(f64),
    Null
}
