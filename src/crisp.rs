use crate::prelude::*;
use std::cell::RefCell;
use std::hash::*;
use std::rc::Rc;

pub struct Crisp<T> {
    heap:      Heap,
    globals:   RefCell<Module<T>>,
    crisplets: RefCell<Vec<Gc<Crisplet<T>>>>,
}

impl<T: TraceTrait + FinalizeTrait<T>> TraceTrait for Crisp<T> {
    fn trace(&self, vis: &mut Visitor) {
        self.globals.borrow().trace(vis);
        for c in self.crisplets.borrow().iter() {
            c.trace(vis);
        }
    }
}

impl<T: FinalizeTrait<T> + TraceTrait + 'static> Crisp<T> {
    /// Creates a new Crisp system, isolated from all others.
    pub fn new() -> Rc<Self> {
        Rc::new(Crisp {
            heap:      Heap::default(),
            globals:   RefCell::new(Module::new_world()),
            crisplets: RefCell::new(Vec::new()),
        })
    }

    pub fn crisplet(
        self: &Rc<Self>,
        env: Env<T>
    ) -> Result<Gc<Crisplet<T>>, RuntimeError<Crisplet<T>>> {
        unsafe {
            self.gc_ref(Crisplet::new(self.clone(), env))
        }.map_err(RuntimeError::OutOfMemory)
    }

    /// Attempts to find a bound symbol in the global environment.
    pub fn lookup_global(&self, name: &Symbol) -> Option<Gc<T>> {
        self.globals.borrow().lookup(name)
    }

    /// # Safety
    ///
    /// This is only guaranteed to point correctly until the next
    /// garbage collection. To deal with this, you must push it onto a
    /// crisplet's stack so it will be kept alive. Failing that, you
    /// must at least mark it before a gc occurs, although it's hard
    /// to know when that will be before it happens...
    pub unsafe fn gc_ref<U>(&self, value: U) -> Result<Gc<U>, U>
    where U: GCInfoTrait<U> + TraceTrait + FinalizeTrait<U> + 'static {
        let heap = &self.heap;
        match heap.alloc(value) {
            Ok(val) => Ok(val),
            Err(val) => { // If it failed, we can run a collection and try again.
                heap.collect();
                heap.alloc(value)
            }
        }
    }
}

impl<T: Hash> Hash for Crisp<T> {
    fn hash<H: Hasher>(&self, state: &mut H) { self.globals.borrow().hash(state) }
}

pub struct Crisplet<T> {
    crisp: &'static Crisp<T>,
    env:   Gc<Env<T>>,
    stack: Vec<Gc<T>>,
}

impl<T: FinalizeTrait<T> + TraceTrait + 'static> Crisplet<T> {
    pub fn new(crisp: Rc<Crisp<T>>, env: Gc<Env<T>>) -> Self {
        Crisplet { crisp, env, stack: Vec::new() }
    }
    pub fn push(&mut self, value: Gc<T>) {
        self.stack.push(value);
    }
    pub fn push_gc(&mut self, value: T) -> Result<Gc<T>, RuntimeError<T>> {
        let gc = unsafe { self.crisp.gc_ref(value) }.map_err(RuntimeError::OutOfMemory)?;
        self.stack.push(gc);
        Ok(gc)
    }
}

// TODO: need to read the comet code and find out whether our drop
// will be called or we need to do it here. we suspect the latter but
// i cba right now because i am feeling unwell.
impl<T: FinalizeTrait<T>> FinalizeTrait<Crisplet<T>> for Crisplet<T> {
}

impl<T: TraceTrait> TraceTrait for Crisplet<T> {
    fn trace(&self, vis: &mut Visitor) {
       self.env.trace(vis);
        for s in self.stack.iter() {
            s.trace(vis);
        }
    }
}

pub enum State {
    Runnable,
    Running,
    Done,
}

pub struct Crisplets<T> {
    crisplets: RefCell<Vec<Crisplet<T>>>,
}

// TODO: need to read the comet code and find out whether our drop
// will be called or we need to do it here. we suspect the latter but
// i cba right now because i am feeling unwell.
impl<T: FinalizeTrait<T>> FinalizeTrait<Crisplets<T>> for Crisplets<T> {
}

impl<T: TraceTrait> TraceTrait for Crisplets<T> {
    fn trace(&self, vis: &mut Visitor) {
        for c in self.crisplets.borrow().iter() {
            c.trace(vis);
        }
    }
}
