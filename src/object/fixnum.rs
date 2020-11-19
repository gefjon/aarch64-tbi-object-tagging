use super::tag::{self, Word, Tag};

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Fixnum(u64);

impl Word for Fixnum {
    #[inline(always)]
    fn to_u64(self) -> u64 { self.0 }
    #[inline(always)]
    fn from_u64(u: u64) -> Self {
        debug_assert_eq!(tag::extract_tag(u), Tag::Fixnum);
        Fixnum(u)
    }
}

#[inline(always)]
pub fn fixnum_arith(lhs: Fixnum, rhs: Fixnum, op: impl FnOnce(u64, u64) -> u64) -> Fixnum {
    let result = op(lhs.to_u64(), rhs.to_u64());
    debug_assert_eq!(tag::extract_tag(result), Tag::Fixnum);
    Fixnum::from_u64(tag::add_tag(result, Tag::Fixnum))
}
