use std::collections::HashMap;

use crate::Object;
use crate::traits::Container;
use crate::traits::Environment;

#[derive(Debug, Clone)]
pub struct Env {
    store: HashMap<String, Container<Object>>,
    outer: Option<Container<Env>>
}

impl Env {
    pub fn new() -> Self {
        Env {
            store: HashMap::new(),
            outer: None,
        }
    }
}

impl Environment for Env {

    fn get(&self, name: &str) -> Option<Container<Object>> {
        match self.store.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .outer
                .as_ref()
                .and_then(|env| env.get().get(name).clone()),
        }
    }

    fn set(&mut self, name: &str, val: Object) {
        match self.store.get_mut(name) {
            Some(var) => { 
                var.set(val); 
            },
            None => { 
                self.store.insert(
                    String::from(name), 
                    Container::new(val)
                ); 
            }
        }
    }

}