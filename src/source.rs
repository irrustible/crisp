use crate::prelude::*;
use std::hash::*;
use std::path::PathBuf;

#[derive(Debug,Eq,FinalizeTrait,Hash,PartialEq,TraceTrait)]
pub struct Source {
    pub path: Option<PathBuf>,
    pub code: String,
}

impl Source {
    pub fn new(path: Option<PathBuf>, code: String) -> Self {
        Self { path, code }
    }
}


#[derive(Clone,Copy,Debug,Eq,FinalizeTrait,PartialEq,TraceTrait)]
pub struct Span {
    pub file:  Gc<Source>,
    pub start: usize,
    pub end:   usize,
}

impl Span {
    pub fn new(file: Gc<Source>, start: usize, end: usize) -> Self {
        Self { file, start, end }
    }
}

impl Hash for Span {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file.hash(state);
        self.start.hash(state);
        self.end.hash(state)
    }
}
