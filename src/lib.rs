
// todo:
// * structs
//   * record structsf
//   * tuple structs
// * record enum constructors
// * trace over enums
// * trace over structs
// * finalize
//   * drop
// * type parameter defaults
// * const generics (how?)
///
///
/// ```
/// crispr! {
///     #[+Trace,Finalize]
///     pub enum Wat<T> {
///         Foo,
///         Bar( #[notrace] #[drop] Baz<T> ),
///     }
/// }
///
///
///
/// ```
#[macro_export]
macro_rules! crispr {
    // do nothing
    () => {};

    // Matches a struct or enum declaration. Probably
    
    ( $( #[ $($meta:meta )* ] )* // any number of preceding attributes
      $vis:vis $struct:ident $name:ident  // an enum or struct declaration
      $(< $( $lt1:lifetime $( : $lt2:lifetime  $( + $lt3:lifetime )* )? ),*
          $(,)?
          $( $gen1:ident
             $( : $trt1:ident $(< $( $tty1:ty ),* >)?
                  $( + $trt2:ident $(< $( $tty2:ty ),* >)? )*
             )?
          ),*
        >)?   
      { $( $body:tt )* }
    ) => {
        $crate::__crispr! {
            @collate {
                [$( #[ $( $meta )* ] )*],
                [],
                [],
                [ $vis $struct $name
                  $(< $( $lt1 $( : $lt2  $( + $lt3 )* )? ),*
                      $(,)?
                      $( $gen1 $( : $trt1 $(< $( $tty1 ),* >)?
                                    $( + $trt2 $(< $( $tty2 ),* >)? )*
                         )?
                      ),*
                    >)?
                  { $( $body )* }
                ],
            }
        }
    };
}


#[macro_export]
macro_rules! __crispr {

    // step 1: collate(remaining, unrecognised, passes, enum_or_struct_decl)
    //
    // iterate through the struct/enum decl metadata an item at a time
    // and categorise it before we expand the passes

    // base case, move on to the passes
    (@collate {
        [],
        [$( $attrs:tt )*],
        [$( $passes:tt )*],
        [$( $decl:tt )*],
    }) => {
        $crate::__crispr! {
            @expand {
                [$( $passes )*],
                [$( $attrs  )*],
                [$( $decl   )*],
            }
        }
    };

    // new passes case, accrue
    (@collate {
        [ #[+ $( $new:meta )+ ] $( $rest:tt )+ ],
        [$( $attrs:tt )*],
        [$( $passes:tt )*],
        [$( $decl:tt )*],
    }) => {
        $crate::__crispr! {
            @collate {
                [ $( $rest  )+ ],
                [ $( $attrs )* ],
                [ $( $new   )* $( $passes )* ],
                [ $( $decl  )* ],
            }
        }
    };

    // new unrecognised, accrue
    (@collate {
        [ #[ $( $new:meta )+ ] $( $rest:tt )+ ],
        [$( $attrs:tt )*],
        [$( $passes:tt )*],
        [$( $decl:tt )*],
    }) => {
        $crate::__crispr! {
            @collate {
                [ $( $rest )+ ],
                [ #[ $( $new )+ ] $( $attrs )* ],
                [ $( $passes )* ],
                [ $( $decl )* ],
            }
        }
    };

    // step 2: expand

    // base case, move on to the declaration
    (@expand {
        [],
        $( $rest:tt)+
    }) => {
        $crate::__crispr! {
            @decl { $( $rest )+ }
        }
    };

    // todo: a case for each pass
    (@expand {
        [$( $rest:tt )*],
        [$( $attrs:tt )*],
        [$( $decl:tt )*],
    }) => {
        compile_error!("todo: passes")
    };

    (@decl {
        [$( $attrs:tt )*],
        // this is actually all just matching the type name but rust
        // makes life a bit difficult with lifetimes and generics and
        // we aren't even trying to support const generics here.
        [ $vis:vis enum $name:ident
          $(< $( $lt1:lifetime $( : $lt2:lifetime  $( + $lt3:lifetime )* )? ),*
              $(,)?
              $( $gen1:ident
                 $( : $trt1:ident $(< $( $tty1:ty ),* >)?
                      $( + $trt2:ident $(< $( $tty2:ty ),* >)? )*
                 )?
              ),*
            >)?   
          {
              $( $body:tt )*
          }
        ],
    }) => {
        $crate::__crispr! {
            @enum_decl {
                [$( $body  )*],
                [ ],
                [ ],
                [$( $attrs )*
                 $vis enum $name
                 $(< $( $lt1 $( : $lt2  $( + $lt3 )* )? ),*
                     $(,)?
                     $( $gen1 $( : $trt1 $(< $( $tty1 ),* >)?
                                   $( + $trt2 $(< $( $tty2 ),* >)? )*
                        )?
                     ),*
                   >)?   
                ],
            }
        }
    };

    // enum_decl(todo, attrs, ctors, decl)
    //
    // we scan through todo, ultimately producing a list of stripped
    // ctors. there are two recvusions going on here, over the
    // contructors and over the attributes attached to the
    // constructors.
    
    // base case: spit out the enum
    (@enum_decl {
        [ ],
        [ ],
        [ $( $ctors:tt )* ],
        [ $( $decl:tt  )* ],
    }) => {
        $( $decl )* {
            $( $ctors )*
        }
    };

    // notrace: just drop it
    (@enum_decl {
        [ #[notrace] $( $rest:tt )+],
        [ $( $attrs:tt )* ],
        [ $( $ctors:tt )* ],
        [ $( $decl:tt  )* ],
    }) => {
        $crate::__crispr! {
            @enum_decl {
                [ $( $rest  )+ ],
                [ $( $attrs )* ],
                [ $( $ctors )* ],
                [ $( $decl  )* ],
            }
        }
    };

    // leading (formerly trailing) comma, drop it. this is for the
    // tuple ctor case of enum_decl. dealing with the comma here is
    // just easier than battling the ambiguity errors.
    (@enum_decl {
        [ , $( $rest:tt )*],
        [ $( $attrs:tt )* ],
        [ $( $ctors:tt )* ],
        [ $( $decl:tt  )* ],
    }) => {
        $crate::__crispr! {
            @enum_decl {
                [ $( $rest  )* ],
                [ $( $attrs )* ],
                [ $( $ctors )* ],
                [ $( $decl  )* ],
            }
        }
    };

    // unknown: copy it
    (@enum_decl {
        [ #[ $( $meta:meta )+ ] $( $rest:tt )+],
        [ $( $attrs:tt )* ],
        [ $( $ctors:tt )* ],
        [ $( $decl:tt  )* ],
    }) => {
        $crate::__crispr! {
            @enum_decl {
                [ $( $rest  )+ ],
                [ #[ $( $meta )+ ] $( $attrs )* ],
                [ $( $ctors )* ],
                [ $( $decl  )* ],
            }
        }
    };

    // unit ctor at end
    (@enum_decl {
        [ $ctor:ident $(,)?],
        [ $( $attrs:tt )* ],
        [ $( $ctors:tt )* ],
        [ $( $decl:tt  )* ],
    }) => {
        $crate::__crispr! {
            @enum_decl {
                [ ],
                [ ],
                [ $( $ctors )* $( $attrs )* $ctor, ],
                [ $( $decl  )* ],
            }
        }
    };

    // unit ctor not at end
    (@enum_decl {
        [ $ctor:ident, $( $rest:tt )+],
        [ $( $attrs:tt )* ],
        [ $( $ctors:tt )* ],
        [ $( $decl:tt  )* ],
    }) => {
        $crate::__crispr! {
            @enum_decl {
                [ $( $rest  )+ ],
                [ ],
                [ $( $ctors )* $( $attrs )* $ctor(), ],
                [ $( $decl  )* ],
            }
        }
    };

    // tuple ctor
    (@enum_decl {
        [ $ctor:ident( $( $fields:tt )* ) $( $rest:tt )*],
        [ $( $attrs:tt )* ],
        [ $( $ctors:tt )* ],
        [ $( $decl:tt  )* ],
    }) => {
        $crate::__crispr! {
            @tuple_ctor {
                [ $( $fields )* ],
                [ ],
                [ $( $attrs )* $ctor ],
                [ $( $rest  )+ ],
                [ $( $ctors )* ],
                [ $( $decl  )* ],
            }
        }
    };

    // tuple_ctor(
    //    remaining to parse
    //    accrued fields
    //    constructor with attributes
    //    remainder of ctor tokens from enum_decl
    //    accrued constructors
    //    enum declaration
    // )

    // base case, no more fields
    (@tuple_ctor {
        [ ],
        [ $( $fields:tt )* ],
        [ $( $ctor:tt )* ],
        [ $( $enums:tt )* ],
        [ $( $ctors:tt )* ],
        [ $( $decl:tt  )* ],
    }) => {
        $crate::__crispr! {
            @enum_decl {
                [ $( $enums )* ],
                [ ],
                [ $( $ctors )* $( $ctor )* ( $( $fields )* ), ],
                [ $( $decl  )* ],
            }
        }
    };

    // notrace attr, strip
    (@tuple_ctor {
        [ #[notrace] $( $more:tt )+ ],
        [ $( $fields:tt )* ],
        $( $args:tt )*
    }) => {
        $crate::__crispr! {
            @tuple_ctor {
                [ $( $more )+ ],
                [ $( $fields )* ],
                $( $args )*
            }
        }
    };

    // unknown attr, copy
    (@tuple_ctor {
        [ #[$( $meta:meta )*] $( $more:tt )+ ],
        [ $( $fields:tt )* ],
        $( $args:tt )*
    }) => {
        $crate::__crispr! {
            @tuple_ctor {
                [ $( $more )+ ],
                [ $( $fields )* #[ $( $meta )* ] ],
                $( $args )*
            }
        }
    };

    // recursive case: type
    (@tuple_ctor {
        [ $vis:vis $ty:ty $(, $( #[ $( $more_meta:meta )* ] )* $more:ty ),* ],
        [ $( $fields:tt )* ],
        $( $args:tt )*
    }) => {
        $crate::__crispr! {
            @tuple_ctor {
                [ $( $( #[ $( $more_meta )* ] )* $more ),* ],
                [ $( $fields )* $vis $ty, ],
                $( $args )*
            }
        }
    };

}

    // (@item $enum:ident $ctor:ident) => {
    //     impl From<()> for $enum {
    //         fn from(_from: ()) -> Self {
    //             Self::$ctor
    //         }
    //     }
    //     impl Default for $enum {
    //         fn default() -> Self { Self::$ctor }
    //     }
    // };
    // (@item $enum:ident $ctor:ident $type:ty ) => {
    //     impl From<$type> for $enum {
    //         fn from(from: $type) -> Self {
    //             Self::$ctor(from)
    //         }
    //     }
    //     impl core::convert::TryInto<$type> for $enum {
    //         type Error = Self;
    //         fn try_into(self) -> Result<$type, Self> {
    //             match self {
    //                 Self::$ctor(x) => Ok(x),
    //                 _ => Err(self),
    //             }
    //         }
    //     }
    //     impl<'a> core::convert::TryInto<&'a $type> for &'a $enum {
    //         type Error = &'a $enum;
    //         fn try_into(self) -> Result<&'a $type, Self::Error> {
    //             match self {
    //                 $enum::$ctor(x) => Ok(&x),
    //                 _ => Err(self),
    //             }
    //         }
    //     }
    //     impl<'a> core::convert::TryInto<&'a mut $type> for &'a mut $enum {
    //         type Error = &'a mut $enum;
    //         fn try_into(self) -> Result<&'a mut $type, Self::Error> {
    //             match self {
    //                 $enum::$ctor(ref mut x) => Ok(x),
    //                 _ => Err(self),
    //             }
    //         }
    //     }
    // };

crispr! {}

crispr! {
    pub enum Wat {}
}

crispr! {
    pub enum Maybe {
        #[notrace]
        Yes(),
        #[notrace]
        No(bool),
    }
}

crispr! {
    pub enum Dunno {
        #[notrace]
        Yes(),
        #[notrace]
        No(bool),
    }
}

// mod heap;
// use heap::*;

// mod vm;

// pub mod crisp;

// pub mod errors;

// pub mod syntax;

// pub mod name;

// pub mod source;
// use source::*;

// mod prelude;
// pub use prelude::*;

// mod lexicals;

// pub mod env;

// mod stack;

// mod list;

// mod expr;

// pub mod literal;

// pub mod module;

// pub mod scheduler;

// pub mod machine;

// #[derive(Clone,Copy,Debug,Eq,FinalizeTrait,Hash,PartialEq,TraceTrait)]
// pub enum Prim {
//     Define,
//     Defn,
//     Defx,
//     Eval,
//     Fn,
//     Fx,
//     If,
//     Let,
//     Nil,
// }

// #[derive(TraceTrait)]
// pub struct FileScope {
//     it: isize,
// }




/*

* Source file parsed into AST
* AST parsed into High








*/




// pub enum High {
//     Let(GcRef<HighLet>),
    
// }

// pub struct HighLet {
//     bindings: Vec<(GcRef<Symbol>, GcRef<High>)>,
//     body: Box<High>,
// }


// pub enum Low {
// }


// #![feature(type_alias_impl_trait)]
// mod symbol;

// use im_rc::{Vector, HashMap};
// use smartstring::alias::*;
// use std::cell::RefCell;
// use std::convert::TryInto;
// use std::fmt::{self, Debug};
// use std::hash::Hash;
// use std::iter::FromIterator;
// use std::rc::Rc;

// pub mod ast;

// pub mod name;
// use name::*;

// pub mod list;
// use list::*;

// pub mod map;
// use map::*;

// pub mod env;
// use env::*;

// pub mod expr;
// use expr::*;

// pub mod source;
// use source::*;

// pub mod module;
// use module::*;

// pub mod lexicals;
// use lexicals::*;

// pub mod stack;
// use stack::*;

// // pub mod data;


// pub use pretty::RcDoc;

// pub enum Value {
// }

// // pub trait LISP {
// //     type Eval;
// //     type Call;
// //     type Unwind;
// //     fn eval(&self, env: &mut Env) -> Self::Eval;
// //     fn call(&self, list: &List<Expr>, env: &mut Env) -> Self::Call;
// //     fn unwind(&self, env: &mut Env) -> Self::Unwind;
// // }

// // impl LISP for () {
// //     type Eval = ();
// //     type Call = ();
// //     type Unwind = ();
// //     fn eval(&self, env: &mut Env) {}
// //     fn call(&self, list: &List<Expr>, env: &mut Env) {}
// //     fn unwind(&self, env: &mut Env) {}
// // }

// // type Lispy = Box<dyn LISP>;

// pub trait PrettyPrint {
//     fn pp(&self) -> RcDoc<()>;
// }

// pub struct Stack {
//     frames: Vector<StackFrame>,
// }

// #[derive(Clone)]
// pub enum StackFrame {
//     Call(CallFrame),
// }

// #[derive(Clone)]
// pub struct CallFrame {
//     call:   Expr,
//     locals: HashMap<Symbol, Expr>,
// }

// pub struct Call {
//     what: Expr,
//     with: List<Expr>,
// }

// pub enum Error {
//     Undefined,
// }

// // #[async_trait::async_trait]
// // pub trait Lispy : PrettyPrint {
// //     async fn eval(&self, env: &mut Env) -> Result<Expr, Error>;
// //     async fn call(&self, list: &List, env: &mut Env) -> Result<Expr, Error>;
// // }

// // #[derive(Clone, Debug)]
// // pub struct ArgumentError {
// //     pub expected: String,
// //     pub got:      Expr,
// // }

// // impl ArgumentError {
// //     pub fn new(expected: impl Into<String>, got: Expr) -> Self {
// //         ArgumentError { expected: expected.into(), got }
// //     }
// // }


// variant! {
//     #[derive(Clone,Eq,Hash,PartialEq)]
//     pub enum Expr {
//         Nil,
//         Bool(bool),
//         Int(isize),
//         String(String),
//         Symbol(Symbol),
//         List(List<Expr>),
//         // Fexpr(Rc<Fexpr>),
//         // Prim(Prim),
//         // Env(Env),
//         // Custom(Custom),
//     }
// }

// impl Expr {
//     pub fn is_nil(&self) -> bool { matches!(self, Expr::Nil) }
//     pub fn is_bool(&self) -> bool { matches!(self, Expr::Bool(_)) }
//     pub fn is_int(&self) -> bool { matches!(self, Expr::Int(_)) }
//     pub fn is_string(&self) -> bool { matches!(self, Expr::String(_)) }
//     pub fn is_symbol(&self) -> bool { matches!(self, Expr::Symbol(_)) }
//     pub fn is_list(&self) -> bool { matches!(self, Expr::List(_)) }

// //     pub fn is_fexpr(&self) -> bool { matches!(self, Expr::Fexpr(_)) }
// //     pub fn is_prim(&self) -> bool { matches!(self, Expr::Prim(_)) }
// //     pub fn is_env(&self) -> bool { matches!(self, Expr::Env(_)) }
// //     pub fn is_custom(&self) -> bool { matches!(self, Expr::Custom(_)) }

//     pub fn is_truthy(&self) -> bool {
//         match self {
//             Self::Nil => false,
//             Self::Bool(b) => *b,
//             _ => true,
//         }
//     }

// //     pub async fn eval(&self, env: &Env) -> Result<Expr, Error> {
// //         match self {
// //             Expr::Symbol(s) =>
// //                 env.lookup(&s).ok_or(Error::Undefined).map(Clone::clone),
// //             Expr::List(l) =>
// //                 Self::eval_list(l, env).await,
// //             _ => Ok(self.clone()),
// //         }
// //     }

// //     async fn eval_list(list: &List<Expr>, env: &Env) -> Result<Expr, Error> {
// //         if list.is_empty() { return Ok(Expr::List(list.clone())) }
// //         let combiner = list.items.head().unwrap().eval(env).await?;
// //         match combiner {
// // //             Expr::Fexpr(f) => f.call(list.drop(1), env),
// // //             Expr::Prim(p) => p.call(list.drop(1), env),
// // //             Expr::Custom(c) => c.call(list.drop(1), env),
// //             _ => todo!(),
// //         }
// //     }
// }



// fn compile(expr: Expr) {
    
// }

// // pub enum Instruction {
// //     PushLexicalFrame,
// //     PopLexicalFrame,
// // }


// // fn eval(expr: Expr, env: Env) {
// // }

// impl Debug for Expr {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         fmt.write_str("Expr")
//     }
// }

// #[derive(Clone, Copy, Debug)]
// pub enum Type {
//     Nil,
//     Bool,
//     Int,
//     Fun,
//     Prim,
//     Symbol,
//     List,
//     Env,
// }


// #[derive(Clone, Copy, Debug)]
// pub enum Prim {
//     Do,
//     Def,
//     Eval,
//     If,
//     Quote,
//     Fexpr,
//     Wrap,
//     IsNil,
//     IsBool,
//     IsInt,
//     IsString,
//     IsPrim,
//     IsFexpr,
//     IsSymbol,
//     IsList,
//     IsEnv,
// }
    

// impl Callable for Prim {
//     fn call(&self, args: List, env: &mut Env) -> Result<Expr, Error> {
//         let mut iter = args.items.into_iter();
//         match self {
//             Prim::Do    => iter.try_fold(Expr::Nil, |_,e| e.eval(env)),
//             Prim::Def   => {
//                 let name = pop_a(&mut iter, "a symbol naming the definition")?;
//                 let value = iter.next().unwrap_or_default();
//                 if iter.len() == 0 {
//                     let value = value.eval(env)?;
//                     env.define(name, value);
//                     Ok(Expr::Nil)
//                 } else {
//                     Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
//                 }
//             }
//             Prim::If => {
//                 let cond = iter.next().ok_or_else(|| ArgumentError::new("an condition to evaluate", Expr::Nil))?;
//                 let if_true = iter.next().ok_or_else(|| ArgumentError::new("a success expression", Expr::Nil))?;
//                 let if_false = iter.next().unwrap_or_default();
//                 if iter.len() == 0 {
//                     let cond = cond.eval(env)?;
//                     if cond.is_truthy() {
//                         if_true.eval(env)
//                     } else {
//                         if_false.eval(env)
//                     }
//                 } else {
//                     Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
//                 }
//             }
//             Prim::Eval => {
//                 let value = iter.next().ok_or_else(|| ArgumentError::new("an expression to evaluate", Expr::Nil))?;
//                 if let Some(env) = iter.next() {
//                     let mut env = env.try_into()
//                         .map_err(|e| ArgumentError::new("an environment to evaluate in", e))?;
//                     if iter.len() == 0 {
//                         value.eval(&mut env)
//                     } else {
//                         Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
//                     }
//                 } else {
//                     value.eval(env)
//                 }
//             }
//             Prim::Quote => {
//                 let value = iter.next().unwrap_or_default();
//                 if iter.len() == 0 {
//                     Ok(value)
//                 } else {
//                     Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
//                 }
//             }
//             Prim::Fexpr => {
//                 let param_name = pop_a(&mut iter, "a symbol naming the parameter")?;
//                 let env_name = pop_a(&mut iter, "a symbol naming the calling environment")?;
//                 let body = iter.next().unwrap_or_default();
//                 if iter.len() == 0 {
//                     Ok(Expr::Fexpr(Rc::new(Fexpr {
//                         param_name, env_name, body, env: env.clone()
//                     })))
//                 } else {
//                     Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
//                 }
//             }
//             Prim::Wrap  => {
//                 todo!()
//             }
//             Prim::IsNil => {
//                 let item = iter.next()
//                     .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
//                 if iter.len() == 0 {
//                     Ok(Expr::Bool(item.is_nil()))
//                 } else {
//                     Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
//                 }
//             }
//             Prim::IsBool => {
//                 let item = iter.next()
//                     .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
//                 if iter.len() == 0 {
//                     Ok(Expr::Bool(item.is_bool()))
//                 } else {
//                     Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
//                 }
//             }
//             Prim::IsInt => {
//                 let item = iter.next()
//                     .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
//                 if iter.len() == 0 {
//                     Ok(Expr::Bool(item.is_int()))
//                 } else {
//                     Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
//                 }
//             }
//             Prim::IsString => {
//                 let item = iter.next()
//                     .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
//                 if iter.len() == 0 {
//                     Ok(Expr::Bool(item.is_string()))
//                 } else {
//                     Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
//                 }
//             }
//             Prim::IsPrim => {
//                 let item = iter.next()
//                     .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
//                 if iter.len() == 0 {
//                     Ok(Expr::Bool(item.is_prim()))
//                 } else {
//                     Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
//                 }
//             }
//             Prim::IsFexpr => {
//                 let item = iter.next()
//                     .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
//                 if iter.len() == 0 {
//                     Ok(Expr::Bool(item.is_fexpr()))
//                 } else {
//                     Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
//                 }
//             }
//             Prim::IsSymbol => {
//                 let item = iter.next()
//                     .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
//                 if iter.len() == 0 {
//                     Ok(Expr::Bool(item.is_symbol()))
//                 } else {
//                     Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
//                 }
//             }
//             Prim::IsList => {
//                 let item = iter.next()
 //                     .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
//                 if iter.len() == 0 {
//                     Ok(Expr::Bool(item.is_list()))
//                 } else {
//                     Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
//                 }
//             }
//             Prim::IsEnv => {
//                 let item = iter.next()
//                     .ok_or_else(|| ArgumentError::new("a value to test", Expr::Nil))?;
//                 if iter.len() == 0 {
//                     Ok(Expr::Bool(item.is_env()))
//                 } else {
//                     Err(Error::ExtraArguments(List { items: Vector::from_iter(iter) }))
//                 }
//             }
//         }
        
//     }
// }


// #[derive(Clone)]
// pub struct Fexpr {
//     param_name: Symbol,
//     env_name:   Symbol,
//     body:       Expr,
//     env:        Env,
// }

// impl Callable for Fexpr {
//     fn call(&self, args: List, calling_env: &mut Env) -> Result<Expr, Error> {
//         let Fexpr { param_name, env_name, body, mut env } = self.clone();
//         env.entries.insert(param_name, Expr::from(args));
//         env.entries.insert(env_name, Expr::from(calling_env.clone()));
//         body.eval(&mut env)
//     }
// }

// pub struct Lambda(Fexpr);

// // pub mod parser;
// // pub mod source;
// // pub mod builtin;

// #[derive(Debug)]
// pub struct TypeError<T> {
//     expected: &'static str,
//     got:      T,
// }
