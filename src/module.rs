use crate::prelude::*;
use std::hash::*;
use std::collections::HashMap;

#[derive(Clone,Eq,FinalizeTrait,PartialEq,TraceTrait)]
pub struct Module<T> {
    pub name:    Option<Name>,
    pub span:    Option<Span>,
    pub entries: HashMap<Symbol, Gc<T>>,
}

impl<T> Module<T> {
    pub fn new(name: impl Into<Name>, span: Option<Span>) -> Self {
        Module {
            name: Some(name.into()),
            span,
            entries: HashMap::new()
        }
    }
    pub fn new_world() -> Self {
        Module { name: None, span: None, entries: HashMap::new() }
    }

    pub fn lookup(&self, name: &Symbol) -> Option<Gc<T>> {
        self.entries.get(name).map(|x| *x)
    }
    pub fn define(&mut self, name: Symbol, value: Gc<T>) -> Option<Gc<T>> {
        self.entries.insert(name, value)
    }
}

impl<T: Hash> Hash for Module<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.span.hash(state);
        for (k,v) in self.entries.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}
    
