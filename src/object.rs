use std::convert::{TryFrom};

mod fixnum;
mod gc_ptr;
mod tag;
mod typeinfo;
mod vector;
mod word;

use word::Word;
pub use typeinfo::{Boxed, FixedSize, Immediate, TypeId, Type, VariableSize};
pub use fixnum::{Fixnum, signed_arith, unsigned_arith};
pub use gc_ptr::GcPtr;
pub use vector::Vector;

#[repr(transparent)]
#[derive(Copy, Clone)]
struct Object(u64);

impl Word for Object {
    #[inline(always)]
    fn to_u64(self) -> u64 { self.0 }
    #[inline(always)]
    fn from_u64(u: u64) -> Self { Object(u) }
}

impl TryFrom<Object> for Fixnum {
    type Error = TypeId;
    fn try_from(o: Object) -> Result<Fixnum, TypeId> {
        let tag = tag::extract_tag(o);
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
        let tag = tag::extract_tag(o);
        if tag == T::ID {
            Ok(GcPtr::from_u64(o.to_u64()))
        } else {
            Err(tag)
        }
    }
}
