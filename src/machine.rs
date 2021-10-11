use crate::{literal::List, prelude::*};
use std::rc::Rc;

// pub struct Machine<T> {
    
// }


/// The Level 1 intermediate representation is a simple rewriting of
/// the source code with no prior knowledge other than the language
/// semantics. In particular, it knows nothing of the environment in
/// which it is compiled.
pub enum L1<T> {
    /// End crisplet, returning accumulator which is probably an error.
    Halt,
    /// Looks up a lexical into the accumulator, sets next expression
    LookupLexical(LookupLexical<T>),
    /// Sets a value into the accumulator, sets next expression
    Constant(Constant<T>),
    /// If the accumulator is truthy, set next expr to this, else this.
    If(If<T>),
    ///
    Assign(Symbol, T, Rc<L1<T>>),
    /// Return the accumulator to the caller.
    Return,
    // Env
    /// Evaluates the expression
    Eval(T, Rc<L1<T>>),
    
}

pub struct LookupLexical<T> {
    pub name: Symbol,
    pub next: Rc<L1<T>>,
}

pub struct Constant<T> {
    pub value: T,
    pub next: Rc<L1<T>>,
}

pub struct If<T> {
    pub when_true:  Rc<L1<T>>,
    pub when_false: Rc<L1<T>>,
}

pub struct State<T> {
    pub acc: Option<T>,
}

fn ret<T>() -> Rc<L1<T>> { Rc::new(L1::Return) }

pub enum Scope {
    /// Top level in the file
    File,
    ///
    ListHead,
}

/// Given some forms, assumed to represent the body of a function (or
/// a file), compile to the level 1 intermediate representation.
pub fn compile_l1<I>(lits: I)-> Result<L1<Literal>,()>
where I: IntoIterator<Item=Literal>,
      I::IntoIter: ExactSizeIterator + DoubleEndedIterator {
    // End by returning the empty list because we don't have nil yet
    let value = Literal::List(List { values: Vec::new(), prev: None });
    let next = L1::Constant(Constant { value, next: ret() });
    // We're actually going to iterate in reverse because then it's just a simple reduction
    lits.into_iter.reverse().try_fold(next, |next, lit| {
        match lit {
            Literal::Int(i) =>
                Ok(L1::Constant(Constant { value: lit, next })),
            Literal::Quoted(q) =>
                Ok(L1::Constant(Constant { value: q.value, next })),
            Literal::List(l) => {
                if l.values.is_empty() {
                    Ok(L1::Constant(Constant { value: lit, next: ret() }))
                } else {
    
                 todo!()
                }
            }
            _ => todo!(),
        }
    })
}

pub struct L1ify<I> {
    inner: I,
}

// impl<I> Iterator for L1ify<I>
// where I: Iterator<Item=Literal>

/*

(vex call env
  ...)
  









*/
