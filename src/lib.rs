#![feature(
    const_fn,
)]

mod object;

use std::{cell::Cell, ops::Deref};

#[repr(transparent)]
/// A pointer to a thread-local garbage-collected object.
///
/// The pointer in `ptr` must always have its high byte set to `GC_TAG`
pub struct Gc<T> {
    ptr: Cell<*mut T>,
}

impl<T> Deref for Gc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        let ptr = self.ptr.get();
        unsafe { &*ptr }
    }
}
