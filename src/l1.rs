use crate::{literal::List, prelude::*};
use std::iter;

/// The Level 1 intermediate representation is a simple rewriting of
/// the source code with no prior knowledge other than the language
/// semantics. In particular, it knows nothing of the environment in
/// which it is compiled, so we can't identify functions or special
/// forms yet.
///
/// What we can do is identify the order of operations we must perform
/// to evaluate it, which will make running it a bit easier. We only
/// need to do this for the top level, indeed can only do this for the
/// top level without an environment!
///
/// This form may look useless, but it's enough to allow us to spot some types of invalid calls. For example you can't call an int or a string or a 

pub enum L1<T> {
    /// No more code in the file.
    EndOfFile,
    /// Looks up a lexical into the accumulator.
    Lookup(Lookup<T>),
    /// Sets a value into the accumulator.
    Constant(Constant<T>),
    /// Calls the vex in the accumulator with the list that called it.
    Call(Call<T>),
}

/// A lookup of a symbol in the lexical environment.
pub struct Lookup<T> {
    pub name: Symbol,
    pub next: Box<L1<T>>,
}

/// Loading a constant into the accumulator
pub struct Constant<T> {
    pub value: T,
    pub next:  Box<L1<T>>,
}

/// Calling a vex in the accumulator with the list that called it
pub struct Call {
    pub list: T,
    pub next: Box<L1<T>>,
}

/// Given a literal and a next instruction, compile the literal to the
/// level 1 intermediate representation.
pub fn literal_to_l1(value: Literal, next: L1<Literal>)-> Result<L1<Literal>,()> {
    match value {
        // Start off with constants
        Literal::Int(i) =>
            Ok(L1::Constant(Constant { value, next })),
        Literal::String(s) =>
            Ok(L1::Constant(Constant { value, next })),
        // Symbols resolve by lookup
        Literal::Symbol(name) =>
            Ok(L1::Lookup(Lookup { name, next })),
        // Quoted values get unquoted
        Literal::Quoted(q) =>
            Ok(L1::Constant(Constant { value: q.value, next })),
        // Lists are either constants if empty or else calls
        Literal::List(l) => {
            if l.values.is_empty() {
                Ok(L1::Constant(Constant { value, next }))
            } else {
                let head = l.head().unwrap().clone();
                let next = L1::Call(Call { list: value, next });
                literal_to_l1(head, next)
            }
        }
    }
}
