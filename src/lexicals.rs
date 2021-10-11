use crate::prelude::*;
use crate::name::Symbol;
use core::hash::*;
use std::collections::HashMap;

#[derive(Clone,Debug,Eq,FinalizeTrait,PartialEq,TraceTrait)]
pub struct Lexical<T> {
    items: HashMap<Symbol, Gc<T>>,
    prev:  Option<Gc<Lexical<T>>>,
}

impl<T> Default for Lexical<T> {
    fn default() -> Self { Self::new() }
}

impl<T> Lexical<T> {
    pub fn new() -> Self {
        Lexical { items: HashMap::new(), prev: None }
    }
    /// Adds a new frame to the lexical scope.
    pub fn nest(inside: Gc<Self>) -> Self {
        Lexical { items: HashMap::new(), prev: Some(inside) }
    }
    /// Drops the last frame in the lexical scope, if there is one.
    pub fn unnest(&self) -> Option<Gc<Lexical<T>>> {
        self.prev
    }
    pub fn assign(&mut self, name: Symbol, value: Gc<T>) -> Option<Gc<T>> {
        self.items.insert(name, value)
    }
    pub fn unassign(&mut self, name: &Symbol) -> Option<Gc<T>> {
        self.items.remove(name)
    }
    pub fn lookup(&self, name: &Symbol) -> Option<Gc<T>> {
        match self.items.get(name) {
            Some(val) => Some(*val),
            None => { // We will do a little loop so we don't have to recurse
                let mut next = self.prev;
                while let Some(frame) = next {
                    if let Some(val) = frame.items.get(name) {
                        return Some(*val);
                    } else {
                        next = frame.prev;
                    }
                }
                None
            }
        }
    }
}

#[allow(clippy::derive_hash_xor_eq)] // actually piss off
impl<T: Hash> Hash for Lexical<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // We will do a little loop so we don't have to recurse
        let mut next = self.prev;
        while let Some(frame) = next {
            for (k,v) in frame.items.iter() {
                k.hash(state);
                v.hash(state);
            }
            next = frame.prev;
        }
    }
}
