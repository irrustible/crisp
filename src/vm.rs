use crate::prelude::*;

pub struct Task {
    heap: &'static Heap,
}

pub struct TaskRef {
}

pub struct VM {
    heap: Heap,
    tasks: Vec<Option<Task>>
}

pub struct Env{}

pub enum Op<T> {
    Halt,
    LoadLexical(Symbol,T),
    Const(T, T),
    Closure(Env, T, T),
    Check(T, T),
    StoreLexical(Symbol, T),
    Continuation(T),
    Continue(T, Symbol),
    Frame(T, T),
    Arg(T),
    Apply,
    Return,
}
