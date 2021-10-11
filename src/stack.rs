use crate::prelude::*;
use crate::name::*;

/// The dynamic extent of the program.
#[derive(Clone,Debug,Eq,FinalizeTrait,Hash,PartialEq,TraceTrait)]
pub struct Dynamic<T> {
    frame: Trace<T>,
    prev:  Option<Gc<Dynamic<T>>>,
}

#[derive(Clone,Debug,Eq,FinalizeTrait,Hash,PartialEq,TraceTrait)]
pub enum Trace<T> {
    /// Evaluate an expression.
    Eval(EvalFrame<T>),
    /// Call with arguments (non-empty list)
    Call(CallFrame<T>),
    // /// Add a dynamic binding
    // Bind(BindFrame<T>),
}

#[derive(Clone,Debug,Eq,FinalizeTrait,Hash,PartialEq,TraceTrait)]
pub struct CallFrame<T> {
    pub call: Gc<T>,
    // pub args: Gc<List<T>>,
}

#[derive(Clone,Debug,Eq,FinalizeTrait,Hash,PartialEq,TraceTrait)]
pub struct EvalFrame<T> {
    pub expr: Gc<T>,
}

// #[derive(Clone,Debug,Eq,FinalizeTrait,Hash,PartialEq,TraceTrait)]
// pub struct BindFrame<T> {
//     pub binding: Name,
//     /// This value is to be restored when the frame is unwound
//     pub prev:    Gc<T>,
// }
