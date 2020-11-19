use std::convert::{TryFrom};

mod tag;
use tag::{Tag, Word, extract_tag};
mod fixnum;
pub use fixnum::Fixnum;
mod gc_ptr;
pub use gc_ptr::GcPtr;

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
    type Error = Tag;
    fn try_from(o: Object) -> Result<Fixnum, Tag> {
        let tag = extract_tag(o);
        if tag == Tag::Fixnum {
            Ok(Fixnum::from_u64(o.to_u64()))
        } else {
            Err(tag)
        }
    }
}
