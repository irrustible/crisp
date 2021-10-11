use crate::crispr;
use crate::prelude::*;

// crispr! {
    /// At this stage, we have stripped out comments and are left with
    /// what you would normally think of as lisp code, albeit potentially
    /// annotated with its source syntax.
    // #[+TryInto]
    // #[derive(FinalizeTrait,TraceTrait)]
    pub enum Literal {
        Int(Int),
        String(Str),
        Symbol(Symbol),
        Quoted(Quoted),
        // Dotted(Dotted),
        List(List),
        // Vect(Vect),
        // Map(Map),
    }
// }

#[derive(FinalizeTrait,TraceTrait)]
pub struct Int {
    pub prev:  Syntax,
    pub value: isize,
}

#[derive(FinalizeTrait,TraceTrait)]
pub struct Str {
    pub prev:  Syntax,
    pub value: String,
}

#[derive(FinalizeTrait,TraceTrait)]
pub struct Quoted {
    pub prev:  Syntax,
    pub value: Literal,
}

#[derive(FinalizeTrait,TraceTrait)]
pub struct List {
    pub prev:   Syntax,
    pub values: Vec<Literal>,
}

