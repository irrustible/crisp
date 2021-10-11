use crate::prelude::*;
pub use comet::Config;
pub use comet::internal::{
    gc_info::GCInfoTrait,
    trace_trait::TraceTrait,
    finalize_trait::FinalizeTrait,
};
pub use comet::visitor::Visitor;
use comet::gcref::{GcRef, UntypedGcRef};
use std::ops::{Deref,DerefMut};
use std::cell::RefCell;
use std::hash::*;
use std::marker::PhantomData;
use std::mem::size_of;

pub struct Heap {
    inner: RefCell<Box<comet::heap::Heap>>,
}

impl Eq for Heap {}

impl PartialEq for Heap {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl TraceTrait for Heap {}

impl Default for Heap {
    fn default() -> Self { Self::new(Config::default()) }
}

impl Heap {

    pub fn new(config: Config) -> Self {
        Heap { inner: RefCell::new(comet::heap::Heap::new(config)) }
    }

    /// ## Safety
    #[inline(always)]
    pub unsafe fn uninit<T>(&self) -> Option<Uninit<T>>
    where T: GCInfoTrait<T> + TraceTrait + FinalizeTrait<T> + 'static {
        let size = size_of::<T>();
        let inner = self.inner.borrow_mut().allocate_raw(size, T::index())?;
        Some(Uninit { inner, _phan: PhantomData })
    }

    /// ## Safety
    #[inline(always)]
    pub unsafe fn alloc<T>(&self, value: T) -> Result<Gc<T>, T>
    where T: GCInfoTrait<T> + TraceTrait + FinalizeTrait<T> + 'static {
        match self.uninit() {
            Some(u) => Ok(u.init(value)),
            None => Err(value),
        }
    }

    /// ## Safety
    #[inline(always)]
    pub unsafe fn alloc_default<T>(&self) -> Option<Gc<T>>
    where T: Default + GCInfoTrait<T> + TraceTrait + FinalizeTrait<T> + 'static {
        Some(self.uninit()?.init(T::default()))
    }

    pub fn collect(&self) { self.inner.borrow_mut().collect_garbage() }

    pub fn maybe_collect(&self) { self.inner.borrow_mut().collect_if_necessary_or_defer() }
}

#[repr(transparent)]
pub struct Uninit<T> {
    inner: UntypedGcRef,
    _phan: PhantomData<*mut T>,
}

impl<T: GCInfoTrait<T> + TraceTrait + FinalizeTrait<T> + 'static> Uninit<T> {
    /// ## Safety
    ///
    /// You must not use the returned value after the Heap is dropped.
    #[inline(always)]
    pub unsafe fn init(self, with: T) -> Gc<T> {
        self.inner.get().cast::<T>().write(with);
        Gc { inner: self.inner.cast_unchecked::<T>() }
    }
}

#[repr(transparent)]
#[derive(Debug,Eq,PartialEq,TraceTrait)]
pub struct Gc<T> {
    inner: GcRef<T>,
}

impl<T> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Gc { inner: self.inner }
    }
}

impl<T> Copy for Gc<T> {}

impl<T> From<GcRef<T>> for Gc<T> {
    fn from(inner: GcRef<T>) -> Self { Gc { inner } }
}

impl<T> From<Gc<T>> for GcRef<T> {
    fn from(gc: Gc<T>) -> Self { gc.inner }
}

impl<T> Deref for Gc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T> DerefMut for Gc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

#[allow(clippy::derive_hash_xor_eq)] // actually piss off
impl<T: Hash> Hash for Gc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.deref().hash(state)
    }
}
