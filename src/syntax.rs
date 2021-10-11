use crate::*;
use crate::prelude::*;

pub enum Inert {
    LineComment(LineComment),
}

/// This is the raw syntax read from a file. It includes line comments
/// as well as code.
#[derive(FinalizeTrait,TraceTrait)]
pub enum Syntax {
    LineComment(LineComment),
    Int(Int),
    String(Str),
    Symbol(Symbol),
    // Quoted(Quoted),
    // Dotted(Dotted),
    List(List),
    // Vect(Vector),
    // Map(Map),
}

#[derive(FinalizeTrait,TraceTrait)]
pub struct LineComment {
    pub delim:      String,
    pub delim_span: Span,
    pub data_span:  Span,
    pub data:       String,
}

#[derive(FinalizeTrait,TraceTrait)]
pub struct Quote {
    pub span:   Span,
}

#[derive(FinalizeTrait,TraceTrait)]
pub struct Int {
    pub span:  Span,
    pub value: isize,
}

// #[derive(FinalizeTrait,TraceTrait)]
// pub struct Dotted {
//     pub span:  Span,
//     pub left:  Option<Box<Syntax>>,
//     pub right: Option<Box<Syntax>>,
// }

#[derive(FinalizeTrait,TraceTrait)]
pub struct Str {
    pub open:  Span,
    pub close: Span,
    pub raw:   String,
    pub span:  Span,
}

#[derive(FinalizeTrait,TraceTrait)]
pub struct Symbol {
    pub val:  String,
    pub span: Span,
}

#[derive(FinalizeTrait,TraceTrait)]
pub struct List {
    pub open:  Span,
    pub close: Span,
    pub items: Vec<Syntax>,
}

// pub struct Vector {
//     pub open:  Span,
//     pub close: Span,
//     pub items: Vec<Syntax>
// }

// pub struct Map {
//     pub open:  Span,
//     pub close: Span,
//     pub items: Vec<Syntax>
// }
 
