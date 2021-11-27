use crate::source::*;
use crate::data::*;
use im_rc::{HashMap, Vector};
use std::io;
use std::ops::Deref;
use std::rc::Rc;

pub enum Error {
    Undefined(Datum, Env),
    NotCallable(Datum, Option<(Span, Span)>),
    TooFewArgs(Datum, Vec<Datum>, Params),
    TooManyArgs(Datum, Vec<Datum>, Params),
    ExpectedBinding(Datum),
    IO(io::Error),
}

pub fn eval(env: &Env, datum: &Datum) -> Result<Datum, Error> {
    match datum {
        Datum::Symbol(s,_) => match env.lookup(s) {
            Some(val) => Ok(val),
            None => Err(Error::Undefined(datum.clone(), env.clone())),
        }
        Datum::List(l, span) => eval_list(env, l, span),
        Datum::VecLit(l,span) => {
            let items = l.iter().try_fold(Vector::new(), |mut acc, d| {
                acc.push_back(eval(env,d)?);
                Ok(acc)
            })?;
            Ok(Datum::Vec(items, span.clone()))
        }
        Datum::MapLit(l, span) => {
            let items = l.iter().try_fold(HashMap::new(), |mut acc, (k,v)| {
                acc.insert(eval(env,k)?, eval(env,v)?);
                Ok(acc)
            })?;
            Ok(Datum::Map(items, span.clone()))
        }
        _ => Ok(datum.clone()),
    }
}

fn eval_list(env: &Env, list: &[Datum], span: &Option<(Span, Span)>) -> Result<Datum, Error> {
    match list {
        [] => Ok(Datum::List(list.to_vec(), span.clone())),
        [head, rest @ ..] => {
            match eval(env, head)? {
                Datum::Vex(v) => eval_call_vex(env, &v, list, span),
                Datum::Fun(f) => eval_call_fun(env, &f, rest, span),
                // TODO lookups
                // Datum::Keyword(k) => todo!(),
                // Datum::Vec(v,span) => todo!(),
                // Datum::Map(m,span) => todo!(),
                Datum::Prim(p) => eval_call_prim(env, p, rest, span),
                other => Err(Error::NotCallable(other, span.clone())),
            }
        }
    }
}

fn eval_call_fun(env: &Env, fun: &Rc<Fun>, args: &[Datum], span: &Option<(Span, Span)>) -> Result<Datum, Error> {
    // evaluate the arguments
    let mut args = args.iter().try_fold(Vec::new(), |mut acc, a| {
        acc.push(eval(env, a)?);
        Ok(acc)
    })?;
    // create a new frame based on the function's closure and bind the arguments.
    let env = fun.closure.frame();
    let argc = args.len();
    let paramc = fun.params.fixed.len();
    if let Some(scoop) = &fun.params.scoop {
        if paramc <= argc {
            fun.params.fixed.iter().zip(args.drain(0..fun.params.fixed.len())).for_each(|(p,a)| {
                if let Some(name) = &p.name { env.define(name.clone(), a); }
            });
            if let Some(name) = &scoop.name {
                env.define(name.clone(), Datum::List(args, span.clone()));
            }
        } else {
            todo!() // return Err(...)
        }
    } else if paramc == argc {
        fun.params.fixed.iter().zip(args.drain(0..fun.params.fixed.len())).for_each(|(p,a)| {
            if let Some(name) = &p.name { env.define(name.clone(), a); }
        });
    } else {
        todo!() // return Err(...)
    }
    if let Some(call_span) = &fun.params.call_span {
        if let Some(name) = &call_span.name {
            if let Some(span) = span {
                env.define(name.clone(), Datum::SurroundSpan(span.0.clone(), span.1.clone()));
            }
        }
    }
    // evaluate the body
    fun.body.iter().try_fold(Datum::Nil, |_, d| eval(&env, d))
}

fn eval_call_vex(caller: &Env, vex: &Rc<Vex>, args: &[Datum], span: &Option<(Span, Span)>) -> Result<Datum, Error> {
    // create a new frame based on the vex's closure and bind the arguments.
    let env = vex.closure.frame();
    let argc = args.len();
    let paramc = vex.params.fixed.len();
    if let Some(scoop) = &vex.params.scoop {
        if paramc <= argc {
            vex.params.fixed.iter().zip(args.iter()).for_each(|(p, a)| {
                if let Some(name) = &p.name { env.define(name.clone(), a.clone()); }
            });
            if let Some(name) = &scoop.name {
                env.define(name.clone(), Datum::List(args[paramc..].to_vec(), span.clone()));
            }
        } else {
            todo!() // return Err(...)
        }
    } else if paramc == argc {
        vex.params.fixed.iter().zip(args.iter()).for_each(|(p,a)| {
            if let Some(name) = &p.name { env.define(name.clone(), a.clone()); }
        });
    } else {
        todo!() // return Err(...)
    }
    // bind the calling environment
    if let Some(name) = &vex.env.name {
        env.define(name.clone(), Datum::Env(caller.clone()));
    }
    if let Some(call_span) = &vex.params.call_span {
        if let Some(name) = &call_span.name {
            if let Some(span) = span {
                env.define(name.clone(), Datum::SurroundSpan(span.0.clone(), span.1.clone()));
            }
        }
    }
    // evaluate the body
    vex.body.iter().try_fold(Datum::Nil, |_, d| eval(&env, d))
}

fn eval_call_prim(env: &Env, prim: Prim, args: &[Datum], span: &Option<(Span, Span)>) -> Result<Datum, Error> {
    match prim {
        Prim::Vex => eval_prim_vex(env, args, span),
        Prim::Fn => eval_prim_fun(env, args, span),
        Prim::If => match args {
            [cond,when_true] =>
                if eval(env, cond)?.is_truthy() { eval(env, when_true) } else { Ok(Datum::Nil) },
            [cond,when_true, when_false] =>
                if eval(env, cond)?.is_truthy() { eval(env, when_true) } else { eval(env, when_false) },
            [_,_,_, _too_many @ ..] => todo!(), // Err too many
            _ => todo!(), // Err too few
        },
        Prim::Define => match args {
            [name, value] => match name {
                Datum::Symbol(name, _) => {
                    env.define(name.deref().clone(), eval(env, value)?);
                    Ok(Datum::Nil)
                },
                _ => Err(Error::ExpectedBinding(name.clone())),
            }
            [_, _, _too_many @ ..] => todo!(), // Err too many
            _ => todo!(), // Err too few
        },
        // Prim::Import => todo!(),
        // Prim::Def => todo!(),
        // Prim::Defv => todo!(),
        // Prim::Defn => todo!(),
        // Prim::Spawn => todo!(),
        Prim::Eval => match args {
            // this looks weird, but think about it: you will be
            // calling this with symbols most likely, so they will
            // first need to be resolved in the current environment.
            [expr] => eval(env, &eval(env, expr)?),
            [expr, env2] => {
                let expr = eval(env, expr)?;
                let env2 = eval(env, env2)?;
                match &env2 {
                    Datum::Env(e) => eval(e, &expr),
                    _ => todo!(), // expected an environment
                }
            }
            [_, _, _too_many @ ..] => todo!(),
            _ => todo!(),
        }
    }
}

fn eval_prim_fun(env: &Env, args: &[Datum], call_span: &Option<(Span,Span)>) -> Result<Datum, Error> {
    match args {
        [Datum::VecLit(v, span),
         body  @ ..
        ] => Ok(Datum::Fun(Rc::new(Fun {
            params: parse_params_slice(v.as_slice(), span)?,
            closure: env.clone(),
            body: body.to_vec(),
            span: call_span.clone(),
        }))),
        [Datum::Vec(v, span),
         body  @ ..
        ] => Ok(Datum::Fun(Rc::new(Fun {
            params: parse_params_vector(v, span)?,
            closure: env.clone(),
            body: body.to_vec(),
            span: call_span.clone(),
        }))),
        [_invalid_params,
         ..
        ] => todo!(),
        _ => todo!(),
    }
}

fn eval_prim_vex(env: &Env, args: &[Datum], span: &Option<(Span,Span)>) -> Result<Datum, Error> {
    todo!()
}

fn parse_params_slice(items: &[Datum], span: &Option<(Span,Span)>) -> Result<Params, Error> {
    todo!()
}

fn parse_params_vector(items: &Vector<Datum>, span: &Option<(Span,Span)>) -> Result<Params, Error> {
    todo!()
}
