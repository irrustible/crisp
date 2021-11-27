use std::hash::*;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug,Eq,Hash,PartialEq)]
pub struct Source {
    pub path: Option<PathBuf>,
    pub code: String,
}

impl Source {
    pub fn new(path: Option<PathBuf>, code: String) -> Self {
        Self { path, code }
    }
}


#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Span {
    pub file:  Rc<Source>,
    pub start: usize,
    pub end:   usize,
}

impl Span {
    pub fn new(file: Rc<Source>, start: usize, end: usize) -> Self {
        Self { file, start, end }
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Span {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file.hash(state);
        self.start.hash(state);
        self.end.hash(state);
    }
}
