use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::object::Object;

pub type Var = Rc<RefCell<Object>>;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Var>,
    output: HashMap<String, Var>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            output: HashMap::new(),
            outer: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<Var> {
        match self.store.get(name) {
            Some(value) => Some(Rc::clone(&value)),
            None => self
                .outer
                .as_ref()
                .and_then(|env| env.borrow().get(name).clone()),
        }
    }

    pub fn set(&mut self, name: &str, val: Object) {
        match self.store.get(name) {
            Some(var) => { 
                *var.borrow_mut() = val; 
            },
            None => { 
                self.store.insert(
                    String::from(name), 
                    Rc::new(RefCell::new(val))
                ); 
            }
        }
    }

    pub fn get_output(&self, name: &str) -> Option<Var> {
        match self.output.get(name) {
            Some(value) => Some(Rc::clone(&value)),
            None => self
                .outer
                .as_ref()
                .and_then(|env| env.borrow().get(name).clone()),
        }
    }

    pub fn set_output(&mut self, name: &str, val: Object) {
        match self.output.get(name) {
            Some(var) => { 
                *var.borrow_mut() = val; 
            },
            None => { 
                self.store.insert(
                    String::from(name),
                    Rc::new(RefCell::new(val))
                ); 
            }
        }
    }

}