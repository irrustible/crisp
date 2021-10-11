use crate::*;
use std::hash::*;

#[derive(Clone,Eq,PartialEq)]
pub struct Map {
    inner: Rc<RefCell<Inner>>
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.borrow().hash(state)
    }
}

#[derive(Clone,Eq,Hash,PartialEq)]
struct Inner {
    values: HashMap<Expr, Expr>,
    span:   Option<Span>,
}
