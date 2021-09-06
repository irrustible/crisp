extern crate alloc;
// mod symbol;

use im_rc::{Vector, HashMap};
use smartstring::alias::*;
use alloc::rc::Rc;
use core::convert::TryInto;
use core::fmt::{self, Debug};
use core::iter::FromIterator;
use core::ops::Deref;

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub struct Symbol(String);

impl Symbol {
    pub fn new(string: impl Into<String>) -> Self { Symbol(string.into()) }
    pub fn as_str(&self) -> &str { self.0.as_str() }
}

#[derive(Clone)]
pub struct Env {
    entries: HashMap<Symbol, Expr>,
}

impl Env {
    fn lookup(&self, key: &Symbol) -> Result<Expr, Error> {
        self.entries.get(key).map(Clone::clone).ok_or_else(|| Error::Undefined(key.clone()))
    }
    fn define(&mut self, key: Symbol, value: Expr) -> Option<Expr> {
        self.entries.insert(key, value)
    }
}

impl Debug for Env {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("Env")
    }
}


#[derive(Clone, Debug)]
pub struct ArgumentError {
    pub expected: String,
    pub got:      Expr,
}

impl ArgumentError {
    pub fn new(expected: impl Into<String>, got: Expr) -> Self {
        ArgumentError { expected: expected.into(), got }
    }
}

macro_rules! variant {
    (pub enum $name:ident {
        $($ctor:ident $( ( $type:ty ) )? $(,)?),*
    }) => {
        #[derive(Clone)]
        pub enum $name {
            $($ctor $( ( $type ) )?),*
        }
        $( variant! { @item $name $ctor $( $type )?  })*
    };
    (@item $enum:ident $ctor:ident) => {
        impl From<()> for $enum {
            fn from(_from: ()) -> Self {
                Self::$ctor
            }
        }
        impl Default for $enum {
            fn default() -> Self { Self::$ctor }
        }
    };
    (@item $enum:ident $ctor:ident $type:ty ) => {
        impl From<$type> for $enum {
            fn from(from: $type) -> Self {
                Self::$ctor(from)
            }
        }
        impl TryInto<$type> for $enum {
            type Error = Self;
            fn try_into(self) -> Result<$type, Self> {
                match self {
                    Self::$ctor(x) => Ok(x),
                    _ => Err(self),
                }
            }
        }
        impl<'a> TryInto<&'a $type> for &'a $enum {
            type Error = &'a $enum;
            fn try_into(self) -> Result<&'a $type, Self::Error> {
                match self {
                    $enum::$ctor(x) => Ok(&x),
                    _ => Err(self),
                }
            }
        }
        impl<'a> TryInto<&'a mut $type> for &'a mut $enum {
            type Error = &'a mut $enum;
            fn try_into(self) -> Result<&'a mut $type, Self::Error> {
                match self {
                    $enum::$ctor(ref mut x) => Ok(x),
                    _ => Err(self),
                }
            }
        }
    };
}

variant! {
    pub enum Error {
        Undefined(Symbol),
        ArgumentError(ArgumentError),
        ExtraArguments(List),
    }
}


#[derive(Clone,Default)]
pub struct List {
    items: Vector<Expr>,
}

impl List {
    pub fn new() -> Self { Self::default() }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn drop(&self, count: usize) -> Self { List { items: self.items.skip(count) } }
    pub fn take(&self, count: usize) -> Self { List { items: self.items.take(count) } }
    pub fn head(&self) -> Expr { self.items.head().map(Clone::clone).unwrap_or_default() }
    pub fn tail(&self) -> Self { self.drop(1) }
    pub fn cons(&self, item: Expr) -> Self {
        let mut new = self.clone();
        new.items.push_front(item);
        new
    }
    pub fn concat(&self, other: List) -> Self {
        let mut new = self.clone();
        new.items.append(other.items);
        new
    }
}

fn pop_a<T>(items: &mut impl Iterator<Item=Expr>, exp: &'static str) -> Result<T, Error>
where Expr: TryInto<T, Error=Expr> {
    let next = items.next().ok_or_else(|| Error::from(ArgumentError::new(exp, Expr::Nil)))?;
    next.try_into().map_err(|e| Error::from(ArgumentError::new(exp, e)))
}


impl Debug for List {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("List")
    }
}

pub trait Callable {
    fn call(&self, args: List, env: &mut Env) -> Result<Expr, Error>;
}

pub type Custom = Rc<dyn Callable>;


variant! {
    pub enum Expr {
        Nil,
        Bool(bool),
        Int(isize),
        String(String),
        Fexpr(Rc<Fexpr>),
        Prim(Prim),
        Symbol(Symbol),
        List(List),
        Env(Env),
        Custom(Custom),
    }
}

impl Expr {
    pub fn is_nil(&self) -> bool { matches!(self, Expr::Nil) }
    pub fn is_bool(&self) -> bool { matches!(self, Expr::Bool(_)) }
    pub fn is_int(&self) -> bool { matches!(self, Expr::Int(_)) }
    pub fn is_string(&self) -> bool { matches!(self, Expr::String(_)) }
    pub fn is_fexpr(&self) -> bool { matches!(self, Expr::Fexpr(_)) }
    pub fn is_prim(&self) -> bool { matches!(self, Expr::Prim(_)) }
    pub fn is_symbol(&self) -> bool { matches!(self, Expr::Symbol(_)) }
    pub fn is_list(&self) -> bool { matches!(self, Expr::List(_)) }
    pub fn is_env(&self) -> bool { matches!(self, Expr::Env(_)) }
    pub fn is_custom(&self) -> bool { matches!(self, Expr::Custom(_)) }
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Bool(b) => *b,
            _ => true,
        }
    }

    pub fn eval(self, env: &mut Env) -> Result<Expr, Error> {
        match self {
            Expr::Symbol(s) => env.lookup(&s),
            Expr::List(l) => Self::eval_list(l, env),
            _ => Ok(self),
        }
    }

    pub fn eval_by_ref(&self, env: &mut Env) -> Result<Expr, Error> {
        match self {
            Expr::Symbol(s) => env.lookup(s),
            Expr::List(l) => Self::eval_list(l.clone(), env),
            _ => Ok(self.clone()),
        }
    }

    fn eval_list(list: List, env: &mut Env) -> Result<Expr, Error> {
        if list.is_empty() { return Ok(Expr::List(list)) }
        let combiner = list.items.head().unwrap().eval_by_ref(env)?;
        match combiner {
            Expr::Fexpr(f) => f.call(list.drop(1), env),
            Expr::Prim(p) => p.call(list.drop(1), env),
            Expr::Custom(c) => c.call(list.drop(1), env),
            _ => todo!(),
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("Expr")
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Type {
    Nil,
    Bool,
    Int,
    Fun,
    Prim,
    Symbol,
    List,
    Env,
}


#[derive(Clone, Copy, Debug)]
pub enum Prim {
    Do,
    Def,
    Eval,
    If,
    Quote,
    Fexpr,
    Wrap,
    IsNil,
    IsBool,
    IsInt,
    IsString,
    IsPrim,
    IsFexpr,
    IsSymbol,
    IsList,
    IsEnv,
}
    

impl Callable for Prim {
    fn call(&self, args: List, env: &mut Env) -> Result<Expr, Error> {
        let mut iter = args.items.into_iter();
        match self {
            Prim::Do    => iter.try_fold(Expr::Nil, |_,e| e.eval(env)),
            Prim::Def   => {
                let name = pop_a(&mut iter, "a symbol naming the definition")?;
                let value = iter.next().unwrap_or_default();
                if iter.len() == 0 {
                    let value = value.eval(env)?;
                    env.define(name, value);
                    Ok(Expr::Nil)
                } else {
                    Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
                }
            }
            Prim::If => {
                let cond = iter.next().ok_or_else(|| ArgumentError::new("an condition to evaluate", Expr::Nil))?;
                let if_true = iter.next().ok_or_else(|| ArgumentError::new("a success expression", Expr::Nil))?;
                let if_false = iter.next().unwrap_or_default();
                if iter.len() == 0 {
                    let cond = cond.eval(env)?;
                    if cond.is_truthy() {
                        if_true.eval(env)
                    } else {
                        if_false.eval(env)
                    }
                } else {
                    Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
                }
            }
            Prim::Eval => {
                let value = iter.next().ok_or_else(|| ArgumentError::new("an expression to evaluate", Expr::Nil))?;
                if let Some(env) = iter.next() {
                    let mut env = env.try_into()
                        .map_err(|e| ArgumentError::new("an environment to evaluate in", e))?;
                    if iter.len() == 0 {
                        value.eval(&mut env)
                    } else {
                        Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
                    }
                } else {
                    value.eval(env)
                }
            }
            Prim::Quote => {
                let value = iter.next().unwrap_or_default();
                if iter.len() == 0 {
                    Ok(value)
                } else {
                    Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
                }
            }
            Prim::Fexpr => {
                let param_name = pop_a(&mut iter, "a symbol naming the parameter")?;
                let env_name = pop_a(&mut iter, "a symbol naming the calling environment")?;
                let body = iter.next().unwrap_or_default();
                if iter.len() == 0 {
                    Ok(Expr::Fexpr(Rc::new(Fexpr {
                        param_name, env_name, body, env: env.clone()
                    })))
                } else {
                    Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
                }
            }
            Prim::Wrap  => {
                todo!()
            }
            Prim::IsNil => {
                let item = iter.next()
                    .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
                if iter.len() == 0 {
                    Ok(Expr::Bool(item.is_nil()))
                } else {
                    Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
                }
            }
            Prim::IsBool => {
                let item = iter.next()
                    .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
                if iter.len() == 0 {
                    Ok(Expr::Bool(item.is_bool()))
                } else {
                    Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
                }
            }
            Prim::IsInt => {
                let item = iter.next()
                    .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
                if iter.len() == 0 {
                    Ok(Expr::Bool(item.is_int()))
                } else {
                    Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
                }
            }
            Prim::IsString => {
                let item = iter.next()
                    .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
                if iter.len() == 0 {
                    Ok(Expr::Bool(item.is_string()))
                } else {
                    Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
                }
            }
            Prim::IsPrim => {
                let item = iter.next()
                    .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
                if iter.len() == 0 {
                    Ok(Expr::Bool(item.is_prim()))
                } else {
                    Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
                }
            }
            Prim::IsFexpr => {
                let item = iter.next()
                    .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
                if iter.len() == 0 {
                    Ok(Expr::Bool(item.is_fexpr()))
                } else {
                    Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
                }
            }
            Prim::IsSymbol => {
                let item = iter.next()
                    .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
                if iter.len() == 0 {
                    Ok(Expr::Bool(item.is_symbol()))
                } else {
                    Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
                }
            }
            Prim::IsList => {
                let item = iter.next()
                    .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
                if iter.len() == 0 {
                    Ok(Expr::Bool(item.is_list()))
                } else {
                    Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
                }
            }
            Prim::IsEnv => {
                let item = iter.next()
                    .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
                if iter.len() == 0 {
                    Ok(Expr::Bool(item.is_env()))
                } else {
                    Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
                }
            }
        }
        
    }
}


#[derive(Clone)]
pub struct Fexpr {
    param_name: Symbol,
    env_name:   Symbol,
    body:       Expr,
    env:        Env,
}

impl Callable for Fexpr {
    fn call(&self, args: List, calling_env: &mut Env) -> Result<Expr, Error> {
        let Fexpr { param_name, env_name, body, mut env } = self.clone();
        env.entries.insert(param_name, Expr::from(args));
        env.entries.insert(env_name, Expr::from(calling_env.clone()));
        body.eval(&mut env)
    }
}

pub struct Lambda(Fexpr);

// pub mod parser;
// pub mod source;
// pub mod builtin;

#[derive(Debug)]
pub struct TypeError<T> {
    expected: &'static str,
    got:      T,
}
