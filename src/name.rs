use crate::prelude::*;

#[derive(Clone,Debug,Eq,FinalizeTrait,Hash,PartialEq,TraceTrait)]
pub struct Symbol {
    pub value: String,
    pub span:  Option<Span>,
}

impl Symbol {
    pub fn new(value: impl Into<String>) -> Self {
        Symbol { value: value.into(), span: None }
    }
    pub fn new_spanning(value: impl Into<String>, span: Span) -> Self {
        Symbol { value: value.into(), span: Some(span) }
    }
    pub fn as_str(&self) -> &str { self.value.as_str() }
}

// impl PrettyPrint for Symbol {
//     fn pp(&self) -> RcDoc<()> {
//         RcDoc::text(self.as_str())
//     }
// }

/// This form is quite flexible, it can encompass all manner of dots.
///
/// ## Examples
///
/// `.`
/// `..`
/// `.a`
/// `.a.b`
/// `.a.b.`
/// `a.`
/// `a.b.`
#[derive(Clone,Debug,Eq,FinalizeTrait,Hash,PartialEq,TraceTrait)]
pub struct Dotted(Vec<DottedItem>);

#[derive(Clone,Debug,Eq,FinalizeTrait,Hash,PartialEq,TraceTrait)]
pub struct Dot(Option<Span>);

#[derive(Clone,Debug,Eq,FinalizeTrait,Hash,PartialEq,TraceTrait)]
pub enum DottedItem {
    Dot(Dot),
    Item(Symbol),
}

#[derive(Clone,Debug,Eq,FinalizeTrait,Hash,PartialEq,TraceTrait)]
pub enum Name {
    Symbol(Symbol),
    Dotted(Dotted),
}

impl From<Symbol> for Name {
    fn from(s: Symbol) -> Name {
        Name::Symbol(s)
    }
}
impl From<Dotted> for Name {
    fn from(d: Dotted) -> Name {
        Name::Dotted(d)
    }
}

