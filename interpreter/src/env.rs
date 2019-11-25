use std::collections::HashMap;

use crate::Container;
use crate::traits::NamespaceTrait;

/// Env
/// 
#[derive(Debug, Clone)]
pub struct Env<O> {
    store: HashMap<String, Container<O>>,
    outer: Option<Container<Env<O>>>
}

impl<O> Env<O> {
    pub fn new() -> Self {
        Env {
            store: HashMap::new(),
            outer: None,
        }
    }
}

impl<O> NamespaceTrait<O> for Env<O> {

    fn get(&self, name: &str) -> Option<Container<O>> {
        match self.store.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .outer
                .as_ref()
                .and_then(|env| env.get().get(name).clone()),
        }
    }

    fn set(&mut self, name: &str, val: O) {
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

#[test]
fn env() {
    use crate::Object;
    
    let mut env = Env::new();
    env.set("test", Object::Bool(true));
    let obj = env.get("test");
    assert!(env.get("test").is_some());
    let obj = obj.unwrap();
    assert_eq!(*obj.get(), Object::Bool(true));
}
