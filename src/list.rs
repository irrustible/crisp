use crate::prelude::*;
use std::iter::*;
use std::fmt::{self, Debug};

pub struct Cons<T> {
    value: Gc<T>,
    next:  Gc<T>,
}

// #[derive(Clone,Debug,Eq,FinalizeTrait,Hash,PartialEq,TraceTrait)]

// pub struct List<Expr> {
//     pub(crate) items: Vec<Gc<Expr>>,
//     pub(crate) span:  Option<Gc<Span>>,
// }

// impl<Expr: Clone> List<Expr> {
//     pub fn new() -> Self { List { items: Vec::new(), span: None } }
//     pub fn unit(item: Expr) -> Self { Self { items: Vec::unit(item), span: None } }
//     pub fn len(&self) -> usize { self.items.len() }
//     pub fn is_empty(&self) -> bool { self.items.is_empty() }
//     pub fn nth(&self, n: usize) -> Expr { self.items.get(n).map(Clone::clone).unwrap_or_default() }
//     pub fn drop(&self, count: usize) -> Self { List { items: self.items.skip(count), span: self.span.clone() } }
//     pub fn take(&self, count: usize) -> Self { List { items: self.items.take(count), span: self.span.clone() } }
//     pub fn head(&self) -> Expr { self.items.head().map(Clone::clone).unwrap_or_default() }
//     pub fn tail(&self) -> Self { self.drop(1) }
//     pub fn cons(&self, item: Expr) -> Self {
//         let mut new = self.clone();
//         new.items.push_front(item);
//         new
//     }
//     pub fn snoc(&self, item: Expr) -> Self {
//         let mut new = self.clone();
//         new.items.push_back(item);
//         new
//     }
//     pub fn concat(&self, other: Self) -> Self {
//         let mut new = self.clone();
//         new.items.append(other.items);
//         new
//     }
//     // pub fn map<F>(&self, f: Expr, env: &mut Env) -> Result<Self, Error> {
//     //     self.items.iter().map(|item| f.call(List::unit(item.clone()), env)).collect()
//     // }
//     // pub fn eval_items(self, env: &mut Env) -> Result<List, Error> {
//     //     self.into_iter().map(|i| i.eval(env)).collect()
//     // }
// }

// impl<Expr: Clone> From<Vec<Expr>> for List<Expr> {
//     fn from(items: Vec<Expr>) -> Self { List { items, span: None } }
// }

// impl<Expr: Clone> FromIterator<Expr> for List<Expr> {
//     fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = Expr> {
//         List { items: Vec ::from_iter(iter), span: None }
//     }
// }

// // impl<Expr: Clone> IntoIterator for List<Expr> {
// //     type Item = Expr;
// //     type IntoIter = im_rc::vector::ConsumingIter<Expr>;
// //     fn into_iter(self) -> Self::IntoIter { self.items.into_iter() }
// // }

// impl<Expr: Clone> Debug for List<Expr> {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         // TODO: ...-ification, deal with long arguments etc.
//         fmt.write_char('(')?;
//         match self.len() {
//             0 => (),
//             1 => self.items[0].fmt(fmt)?,
//             more => {
//                 let mut iter = self.items.iter();
//                 let last = iter.next_back().unwrap();
//                 for i in iter {
//                     i.fmt(fmt)?;
//                     fmt.write_char(" ")?;
//                 }
//                 last.fmt(fmt)?
//             }
//         }
//         fmt.write_char(')')?;
//     }
// }

// pub mod prims {
//     use super::*;

//     /// Creates a list from any provided arguments.
//     ///
//     /// ## Usage
//     ///
//     /// `(list 1 2) ; '(1 2)
//     /// `(list 1)`  ; '(1)
//     /// `(list)`    ; '()
//     pub fn list(args: List, env: &mut Env) -> Result<Expr, Error> {
//         args.eval_items(env).map(Expr::List)
//     }

//     /// length of the list
//     ///
//     /// ## Usage
//     ///
//     /// `(len '(1)) ; 1
//     /// `(len '())  ; 0
//     /// `(len nil)  ; 0
//     pub fn len(args: List, env: &mut Env) -> Result<Expr, Error> {
//         let mut args = limit_args(args, 1)?;
//         let mut args = args.eval_items(env)?;
//         match args.items.pop_front() {
//             Some(Expr::Nil) => Ok(Expr::Int(0)),
//             Some(Expr::List(l)) => Ok(Expr::Int(l.len() as isize)),
//             other => {
//                 let other = other.unwrap_or_default();
//                 Err(Error::ArgumentError(ArgumentError::new("Expected list", other)))
//             }
//         }
//     }

//     /// true is the list is empty, else false.
//     ///
//     /// ## Usage
//     ///
//     /// `(empty? '(1)) ; false
//     /// `(empty? '())  ; true
//     /// `(empty? nil)  ; true
//     pub fn is_empty(args: List, env: &mut Env) -> Result<Expr, Error> {
//         let mut args = limit_args(args, 1)?;
//         let mut args = args.eval_items(env)?;
//         match args.items.pop_front() {
//             Some(Expr::Nil) => Ok(Expr::Bool(true)),
//             Some(Expr::List(l)) => Ok(Expr::Bool(l.is_empty())),
//             other => {
//                 let other = other.unwrap_or_default();
//                 Err(Error::ArgumentError(ArgumentError::new("Expected list", other)))
//             }
//         }
//     }

//     /// Appends all preceding items to the last item, which should be a list.
//     /// In the case of 1 arg, identity, in the case of no args, the empty list.
//     ///
//     /// ## Usage
//     ///
//     /// `(cons 1 2 '(3 4))` ; '(1 2 3 4)
//     /// `(cons '(1 2))      ; '(1 2)
//     /// `(cons '())`        ; '()
//     /// `(cons nil)`        ; '()
//     /// `(cons)`            ; '()
//     pub fn cons(args: List, env: &mut Env) -> Result<Expr, Error> {
//         let mut args = args.eval_items(env)?;
//         match args.items.pop_back() {
//             Some(Expr::Nil) => Ok(Expr::List(args)),
//             Some(Expr::List(l)) => Ok(Expr::List(List::from(args.items + l.items))),
//             other => {
//                 let other = other.unwrap_or_default();
//                 Err(Error::ArgumentError(ArgumentError::new("Expected list", other)))
//             }
//         }
//     }

//     /// Appends all following items to the first item, which should be a list.
//     /// In the case of 1 arg, identity, in the case of no args, the empty list.
//     ///
//     /// ## Usage
//     ///
//     /// `(snoc '(1 2) 3 4)` ; '(1 2 3 4)
//     /// `(snoc '(1 2))      ; '(1 2)
//     /// `(snoc '())`        ; '()
//     /// `(snoc nil)`        ; '()
//     /// `(snoc)`            ; '()
//     pub fn snoc(args: List, env: &mut Env) -> Result<Expr, Error> {
//         let mut args = args.eval_items(env)?;
//         match args.items.pop() {
//             Some(Expr::Nil) => Ok(Expr::List(args)),
//             Some(Expr::List(l)) => Ok(Expr::List(List::from(l.items + args.items))),
//             other => {
//                 let other = other.unwrap_or_default();
//                 Err(Error::ArgumentError(ArgumentError::new("Expected list", other)))
//             }
//         }
//     }

//     /// Concatenates many lists. Treats nil as an empty list.
//     /// In the case of 1 arg, identity, in the case of no args, the empty list.
//     ///
//     /// ## Usage
//     ///
//     /// `(concat '(1 2) '(3) '(4 5))` ; '(1 2 3 4 5)
//     /// `(concat '(1) nil ())         ; '(1)
//     /// `(concat '())`                ; '()
//     /// `(concat)`                    ; '()
//     pub fn concat(args: List, env: &mut Env) -> Result<Expr, Error> {
//         let mut args = args.eval_items(env)?;
//         if args.count() < 2 {
//             Ok(Expr::List(args))
//         } else {
//             let first = match args.next().unwrap() {
//                 Expr::Nil => Expr::List(args),
//                 Expr::List(l) => Ok(l),
//                 other =>
//                     return Err(Error::ArgumentError(ArgumentError::new("Expected list", other))),
//             };
//             args.into_iter().try_fold(|last, next| {
//                 match next {
//                     Some(Expr::Nil) => Ok(Expr::List(args)),
//                     Some(Expr::List(l)) => Ok(Expr::List(List::from(l.items + args.items))),
//                     other => {
//                         let other = other.unwrap_or_default();
//                         Err(Error::ArgumentError(ArgumentError::new("Expected list", other)))
//                     }
//                 }
//             })?
//         }
//     }

//     /// Returns the first item in the list, or nil. Must be provided
//     /// exactly one list. Nil counts as an empty list.
//     ///
//     /// ## Usage
//     ///
//     /// `(head '(1 2 3))` ; 1
//     /// `(head '())`      ; nil
//     /// `(head nil)`      ; nil
//     pub fn head(args: List, env: &mut Env) -> Result<Expr, Error> {
//         let mut args = limit_args(args, 1)?;
//         let mut args = args.eval_items(env)?;
//         match args.items.pop_front() {
//             Some(Expr::Nil) => Ok(Expr::Nil),
//             Some(Expr::List(l)) => Ok(Expr::List(l.head())),
//             other => {
//                 let other = other.unwrap_or_default();
//                 Err(Error::ArgumentError(ArgumentError::new("Expected list", other)))
//             }
//         }
//     }

//     /// Returns all but the first item in the list, or nil. Must be
//     /// provided exactly one list. Nil counts as an empty list.
//     ///
//     /// ## Usage
//     ///
//     /// `(tail '(1 2 3))` ; '(2 3)
//     /// `(tail '())`      ; nil
//     /// `(tail nil)`      ; nil
//     pub fn tail(args: List, env: &mut Env) -> Result<Expr, Error> {
//         let mut args = limit_args(args, 1)?;
//         let mut args = args.eval_items(env)?;
//         match args.items.pop_front() {
//             Some(Expr::Nil) => Ok(Expr::Nil),
//             Some(Expr::List(l)) =>
//                 if l.is_empty() { Ok(Expr::Nil) } else { Ok(Expr::List(l.tail())) },
//             other => {
//                 let other = other.unwrap_or_default();
//                 Err(Error::ArgumentError(ArgumentError::new("Expected list", other)))
//             }
//         }
//     }

//     /// Returns a list consisting of the first n items of the
//     /// list. Nil counts as an empty list. Negative n returns the nth
//     /// item from the end (with 1-based indexing)
//     ///
//     /// ## Usage
//     ///
//     /// `(nth 2  '(1 2 3))` ; 3) ; take the item at index 2
//     /// `(nth -2 '(1 2 3))` ; 2) ; take the second to last item
//     /// `(nth 2  '(1))`     ; nil
//     /// `(nth -2 '(1))`     ; nil
//     /// `(nth 2 '())`       ; nil
//     /// `(nth 2 nil)`       ; nil
//     pub fn nth(args: List, env: &mut Env) -> Result<Expr, Error> {
//         let mut args = limit_args(args, 2)?;
//         let mut args = args.eval_items(env)?;
//         match args.items.pop_front() {
//             Some(Expr::Int(i)) => {
//                 match args.items.pop_front() {
//                     Some(Expr::Nil) => Ok(Expr::Nil),
//                     Some(Expr::List(l)) => {
//                         match i.try_into() {
//                             Ok(n) => Ok(Expr::List(l.get(n))),
//                             _ => {
//                                 let len = l.len() as isize;
//                                 let wanted = isize::min(len, i.abs());
//                                 let drop = len - wanted;
//                                 Ok(l.get(i as usize).map(Clone::clone).map(Expr::List).unwrap_or_default())
//                             }
//                         }
//                     },
//                     other => {
//                         let other = other.unwrap_or_default();
//                         Err(Error::ArgumentError(ArgumentError::new("Expected list", other)))
//                     }
//                 }
//             }
//             other => {
//                 let other = other.unwrap_or_default();
//                 Err(Error::ArgumentError(ArgumentError::new("Expected integer", other)))
//             }
//         }
//     }

//     /// Returns a list consisting of the first n items of the
//     /// list. Nil counts as an empty list. Negative n takes the last
//     /// n items.
//     ///
//     /// ## Usage
//     ///
//     /// `(take 2  '(1 2 3))` ; '(1 2) ; take the first 2 items
//     /// `(take -2 '(1 2 3))` ; '(2 3) ; take the last 2 items
//     /// `(take 2  '(1))`     ; '(1)
//     /// `(take -2 '(1))`     ; '(1)
//     /// `(take 2 '())`       ; '()
//     /// `(take 2 nil)`       ; '()
//     pub fn take(args: List, env: &mut Env) -> Result<Expr, Error> {
//         let mut args = limit_args(args, 2)?;
//         let mut args = args.eval_items(env)?;
//         match args.items.pop_front() {
//             Some(Expr::Int(i)) => {
//                 match args.items.pop_front() {
//                     Some(Expr::Nil) => Ok(Expr::List(args)),
//                     Some(Expr::List(l)) => {
//                         match i.try_into() {
//                             Ok(count) => Ok(Expr::List(i.take(count))),
//                             _ => {
//                                 let len = l.len() as isize;
//                                 let wanted = isize::min(len, i.abs());
//                                 let drop = len - wanted;
//                                 Ok(Expr::List(i.drop(len - wanted)))
//                             }
//                         }
//                     },
//                     other => {
//                         let other = other.unwrap_or_default();
//                         Err(Error::ArgumentError(ArgumentError::new("Expected list", other)))
//                     }
//                 }
//             }
//             other => {
//                 let other = other.unwrap_or_default();
//                 Err(Error::ArgumentError(ArgumentError::new("Expected integer", other)))
//             }
//         }
//     }

//     /// Returns a list consisting of the all elements after the first
//     /// n items of the list. Nil counts as an empty list.
//     ///
//     /// ## Usage
//     ///
//     /// `(drop 2  '(1 2 3))` ; '(3) ; drop the first 2 items
//     /// `(drop -2 '(1 2 3))` ; '(1) ; drop the last 2 items
//     /// `(drop 2  '(1 2))`   ; '()
//     /// `(drop -2 '(1 2))`   ; '()
//     /// `(drop 2 '())`       ; '()
//     /// `(drop 2 nil)`       ; '()
//     pub fn drop(args: List, env: &mut Env) -> Result<Expr, Error> {
//         let mut args = limit_args(args, 2)?;
//         let mut args = args.eval_items(env)?;
//         match args.items.pop_front() {
//             Some(Expr::Int(i)) => {
//                 match args.items.pop_front() {
//                     Some(Expr::Nil) => Ok(Expr::List(args)),
//                     Some(Expr::List(l)) => {
//                         match i.try_into() {
//                             Ok(count) => Ok(Expr::List(i.take(count))),
//                             _ => {
//                                 let len = l.len() as isize;
//                                 let wanted = isize::min(len, i.abs());
//                                 let take = len - wanted;
//                                 Ok(Expr::List(i.take(len - wanted)))
//                             }
//                         }
//                     },
//                     other => {
//                         let other = other.unwrap_or_default();
//                         Err(Error::ArgumentError(ArgumentError::new("Expected list", other)))
//                     }
//                 }
//             }
//             other => {
//                 let other = other.unwrap_or_default();
//                 Err(Error::ArgumentError(ArgumentError::new("Expected integer", other)))
//             }
//         }
//     }
// }
