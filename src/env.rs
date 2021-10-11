use crate::prelude::*;
use std::hash::*;

#[derive(Clone,FinalizeTrait,Hash,Eq,PartialEq,TraceTrait)]
/// The lexical environment, mostly, but also the name of the current module (if any)
pub struct Env<T> {
    pub current_module: Option<Name>,
    lexicals:           Gc<Lexical<T>>,
}

impl<T> Env<T> {
    /// Creates a new environment for computation.
    pub fn new() -> Self {
        Env { current_module: None, lexicals: Lexical::new() }
    }
    pub fn nest(&self) -> Self {
        Env {
            current_module: self.current_module.clone(),
            lexicals:       Lexical::nest(self.lexicals),
        }        
    }
}
