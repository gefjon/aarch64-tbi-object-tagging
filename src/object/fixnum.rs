use super::{Immediate, typeinfo, TypeId, Type, tag, word::Word};
use std::{cmp, ops};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Fixnum(u64);

unsafe impl Type for Fixnum {
    const ID: TypeId = TypeId::Fixnum;
}

unsafe impl Immediate for Fixnum {}

impl Word for Fixnum {
    #[inline(always)]
    fn to_u64(self) -> u64 { self.0 }
    #[inline(always)]
    fn from_u64(u: u64) -> Self {
        debug_assert_eq!(tag::extract_tag(u), Self::ID);
        Fixnum(u)
    }
}

impl Fixnum {
    fn to_i64(self) -> i64 {
        make_signed(self.0)
    }
    fn from_i64(i: i64) -> Self {
        Self::from_u64(make_unsigned(i))
    }
}

const SIGN_BIT: u64 = 1 << 55;
const HIGH_BYTE: u64 = 0xff << 56;

fn sign_bit_set(u: u64) -> bool {
    (u & SIGN_BIT) != 0
}

fn make_signed(u: u64) -> i64 {
    (if sign_bit_set(u) {
        u & HIGH_BYTE
    } else { u }) as i64
}

fn make_unsigned(i: i64) -> u64 {
    let u = i as u64;
    if sign_bit_set(u) {
        debug_assert_eq!(u & HIGH_BYTE, HIGH_BYTE);
        u & !HIGH_BYTE
    } else { u }
}

#[inline(always)]
pub fn unsigned_arith(lhs: Fixnum, rhs: Fixnum, op: impl FnOnce(u64, u64) -> u64) -> Fixnum {
    let result = op(lhs.0, rhs.0);
    let _tag = tag::extract_tag(result);
    debug_assert!((_tag == TypeId::Fixnum));
    Fixnum::from_u64(tag::add_tag(result, Fixnum::ID))
}

#[inline(always)]
pub fn signed_arith(lhs: Fixnum, rhs: Fixnum, op: impl FnOnce(i64, i64) -> i64) -> Fixnum {
    unsigned_arith(lhs, rhs,
                   |lhs, rhs| {
                       make_unsigned(op(make_signed(lhs), make_signed(rhs)))
                   })
}

macro_rules! impl_op {
    ($op:ident $func:ident) => {
        impl ops::$op for Fixnum {
            type Output = Self;
            fn $func(self, rhs: Self) -> Self {
                signed_arith(self, rhs, ops::$op::$func)
            }
        }
    };
    ($($op:ident $func:ident),*$(,)*) => {
        $(impl_op!($op $func);)*
    };
}

impl_op!(
    Add add,
    Sub sub,
    Mul mul,
    Div div,
    BitAnd bitand,
    BitOr bitor,
    BitXor bitxor,
    Rem rem,
    Shl shl,
    Shr shr,
);

impl cmp::PartialOrd for Fixnum {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.to_i64().partial_cmp(&other.to_i64())
    }
}

impl cmp::Ord for Fixnum {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.to_i64().cmp(&other.to_i64())
    }
}

impl cmp::PartialEq<i64> for Fixnum {
    #[inline(always)]
    fn eq(&self, other: &i64) -> bool {
        self.to_i64().eq(other)
    }
}

impl cmp::PartialOrd<i64> for Fixnum {
    #[inline(always)]
    fn partial_cmp(&self, other: &i64) -> Option<cmp::Ordering> {
        self.to_i64().partial_cmp(other)
    }
}
