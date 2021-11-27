use crate::source::*;
use im_rc::{Vector, HashMap};
use std::cell::RefCell;
use std::hash::*;
use std::rc::Rc;

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
pub enum Prim {
    Vex,
    Fn,
    If,
    Eval,
    Define,
    // Import,
    // Def,
    // Defv,
    // Defn,
    // Spawn,
}

#[derive(Clone,Debug,Default,Eq,PartialEq)]
struct Frame {
    bindings: RefCell<Vec<(String, Datum)>>,
}

impl Frame {
    fn define(&self, name: String, value: Datum) {
        self.bindings.borrow_mut().push((name, value));
    }

    fn lookup(&self, name: &str) -> Option<Datum> {
        self.bindings.borrow().iter().rev()
            .filter(|(k,_)| k == name)
            .map(|(_,v)| v.clone())
            .next()
    }
    fn update(&self, name: &str, value: Datum) -> Result<(), Datum> {
        for (k,v) in self.bindings.borrow_mut().iter_mut().rev() {
            if k == name {
                *v = value;
                return Ok(())
            }
        }
        Err(value)
    }
}

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Env {
    frames: Vector<Rc<Frame>>,
}

impl Default for Env {
    fn default() -> Env {
        Env { frames : Vector::unit(Rc::new(Frame::default())) }
    }
}

impl Env {
    pub fn define(&self, name: String, value: Datum) {
        self.frames.last().unwrap().define(name, value);
    }
    pub fn lookup(&self, name: &str) -> Option<Datum>{
        for frame in self.frames.iter().rev() {
            if let Some(x) = frame.lookup(name) {
                return Some(x);
            }
        }
        None
    }
    pub fn frame(&self) -> Env {
        let mut frames = self.frames.clone();
        frames.push_back(Rc::new(Frame::default()));
        Env { frames }
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Env {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for f in &self.frames {
            std::ptr::hash(Rc::as_ptr(f), state);
        }
    }
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub enum Datum {
    Nil,
    Bool(bool, Option<Span>),
    Int(isize, Option<Span>),
    String(Rc<String>, Option<Span>),
    Symbol(Rc<String>, Option<Span>),
    Prim(Prim),
    Keyword(Rc<String>, Option<Span>),
    List(Vec<Datum>, Option<(Span, Span)>),
    Vec(Vector<Datum>, Option<(Span, Span)>),
    Map(HashMap<Datum, Datum>, Option<(Span, Span)>),
    VecLit(Vec<Datum>, Option<(Span, Span)>),
    MapLit(Vector<(Datum, Datum)>, Option<(Span, Span)>),
    Vex(Rc<Vex>),
    Fun(Rc<Fun>),
    Env(Env),
    Span(Span),
    SurroundSpan(Span,Span),
}

impl Datum {
    pub fn is_truthy(&self) -> bool {
        match self {
            Datum::Nil => false,
            Datum::Bool(b,_) => *b,
            _ => true,
        }
    }
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub struct Param {
    pub name: Option<String>,
    pub span: Span,
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub struct Params {
    pub fixed: Vec<Param>,
    pub scoop: Option<Param>,
    pub call_span: Option<Param>,
    pub whole: Option<Param>,
}

impl Params {
    
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub struct Vex {
    pub params: Params,
    pub env: Param,
    pub closure: Env,
    pub body: Vec<Datum>,
    pub span: Option<(Span,Span)>,
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub struct Fun {
    pub params: Params,
    pub closure: Env,
    pub body: Vec<Datum>,
    pub span: Option<(Span,Span)>,
}
