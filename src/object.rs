use std::convert::{TryFrom};

mod fixnum;
mod gc_ptr;
mod tag;
mod typeinfo;
mod array;
mod word;

pub use tag::*;
pub use word::Word;
pub use typeinfo::{Boxed, FixedSize, Immediate, TypeId, Type};
pub use fixnum::{Fixnum, signed_arith, unsigned_arith};
pub use gc_ptr::GcPtr;
pub use array::Array;

use std::cell::Cell;

#[repr(transparent)]
#[derive(Clone)]
pub struct Object {
    word: Cell<u64>,
}

impl Word for Object {
    #[inline(always)]
    fn to_u64(self) -> u64 { self.word.get() }
    #[inline(always)]
    fn from_u64(u: u64) -> Self { Object { word: Cell::new(u) } }
}

impl TryFrom<Object> for Fixnum {
    type Error = TypeId;
    fn try_from(o: Object) -> Result<Fixnum, TypeId> {
        let tag = tag::extract_tag(o.clone());
        if tag == Fixnum::ID {
            Ok(Fixnum::from_u64(o.to_u64()))
        } else {
            Err(tag)
        }
    }
}

impl<T> TryFrom<Object> for GcPtr<T>
where
    T: typeinfo::Boxed,
{
    type Error = TypeId;
    fn try_from(o: Object) -> Result<GcPtr<T>, TypeId> {
        let tag = tag::extract_tag(o.clone());
        if tag == T::ID {
            Ok(GcPtr::from_u64(o.to_u64()))
        } else {
            Err(tag)
        }
    }
}

impl<T: Boxed> From<GcPtr<T>> for Object {
    fn from(ptr: GcPtr<T>) -> Object {
        Object::from_u64(ptr.to_u64())
    }
}

impl From<Fixnum> for Object {
    fn from(fix: Fixnum) -> Object {
        Object::from_u64(fix.to_u64())
    }
}
