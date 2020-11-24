pub trait Word {
    fn to_u64(self) -> u64;
    fn from_u64(u: u64) -> Self;
}

impl Word for u64 {
    #[inline(always)]
    fn to_u64(self) -> u64 { self }
    #[inline(always)]
    fn from_u64(u: u64) -> Self { u }
}

macro_rules! cast_to_word {
    (<$($param:ident),*$(,)*> $($typ:tt)*) => {
        impl<$($param),*> Word for $($typ)* {
            #[inline(always)]
            fn to_u64(self) -> u64 { self as _ }
            #[inline(always)]
            fn from_u64(u: u64) -> Self { u as _ }
        }
    };
    ($typ:ty) => {
        cast_to_word!(<> $typ);
    }
}

cast_to_word!(i64);
cast_to_word!(<T> *mut T);
cast_to_word!(<T> *const T);
