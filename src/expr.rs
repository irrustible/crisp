use crate::prelude::*;

/*

In a normal lisp, evaluation proceeds by identifying special forms by
their name. In a fexpr-based lisp, where operatives (callables) are
first class, we have to evaluate their names before we can know what they are.

Thus, compilation requires a certain amount of evaluation and we may
not always be able to compile everything, meaning we must retain an
interpreter for the cases where we cannot.

This means that the compilation environment matters a lot for any
program involving symbols (i.e. all the useful ones).

Evaluation process:

* Parse source file to Syntax.
* Transform Syntax to Literal, stripping comments out.
* Evaluate the Literal, compiling it as we go along.

(define abc 123)

1. list at top level, so we know we can evaluate the symbol 'define'
2. define is a primitive, so we can compile it.

(vex beige (& _) _ 123)

1. list at top level, so we know we can evaluate the symbol 'vex'
2. vex is a primitive, so we can compile it.
3. 


we need to follow the left spine when evaluating

(((foo 1) 2) 3) ->
  eval foo
  call with (foo 1)
  call with ((foo 1) 2)
  call with (((foo 1) 2) 3)

*/
use std::rc::Rc;

#[derive(Clone)]
pub enum Data {
    Nil,
    Int(isize),
    Symbol(Symbol),
    String(String),
    List(Vec<Data>),
    Env(Env<Data>),
}



// pub enum Form {
//     Fresh(Gc<Literal>),
//     Data(Gc<Literal>),
//     // LexicalReference(Symbol),
// }



// pub enum Effect {
//     DefineNamespace,
//     DefineInNamespace,
// }

// pub struct Node<T> {
//     // things that we depend on
//     upstream:   Vec<Gc<T>>,
//     // things that depend on us
//     downstream: Vec<Gc<T>>,
// }

// pub fn require_library_file(name: Name) {
// }

// pub fn eval_file(

// pub fn eval_fresh(lit: Gc<Literal>, env: Env<Form>) -> Result<Gc<Form>, ()> {
//     match *lit {
//         Literal::Int(i) => Ok(Form::Data(lit)),
//         Literal::String(s) => Ok(Form::Data(lit)),
//         Literal::Symbol(s) =>
//             env.lookup(&s.value).ok_or(()),
//         Literal::Quoted(q) => Ok(Form::Data(q.value)), // todo: maybe prev chain?
//         Literal::List(l) => {
//             if l.values.is_empty() {
//                 Ok(Form::Data(lit))
//             } else {
//                 match l.values.first().unwrap() {
//                     Literal::Int(i) => todo!(),
//                     Literal::String(s) => todo!(),
//                     Literal::Symbol(s) => {
//                         todo!()
//                     }
//                     Literal::Quoted(q) => {
//                         todo!()
//                     }
//                     Literal::List(l) => {
//                         todo!()
//                     }
//                 }
//             }
//         }
//     }
// }

// pub fn eval_file(


// pub fn eval(form: Gc<Form>) {
//     match &*form {
//         Form::Unevaluated(lit) => {
//             todo!()
//         }
//         Form::Data(lit) => {
//             todo!()
//         }
//         // Form::LexicalReference(Symbol) => {
//         //     todo!()
//         // }
//     }
// }

