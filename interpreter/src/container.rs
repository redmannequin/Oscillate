use std::rc::Rc;
use std::cell::Ref;
use std::cell::RefMut;
use std::cell::RefCell;

/// # Container
/// 
#[derive(Debug)]
pub struct Container<T>(Rc<RefCell<T>>);

impl<T> Container<T> {
    pub fn new(item: T) -> Self { Self(Rc::new(RefCell::new(item))) }
    pub fn get(&self) -> Ref<T> { self.0.borrow() }
    pub fn get_mut(&self) -> RefMut<T> { self.0.borrow_mut() }
    pub fn set(&self, item: T) { *self.0.borrow_mut() = item; }
}

impl<T> Clone for Container<T> {
    fn clone(&self) -> Self { Self(Rc::clone(&self.0)) }
}