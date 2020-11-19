use super::{Fixnum, Object, tag::{self, Tag, Word}};
use std::{marker::PhantomData, mem};

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GcPtr<T: ?Sized> {
    ptr: *mut ObjHeader,
    _t: PhantomData<T>,
}

#[repr(C)]
pub struct ObjHeader {
    length: Fixnum,
    /// note: there may be more than one object here, that is, we may
    /// effectively cast `&self.body` into a pointer to `[Object;
    /// self.length]`.
    ///
    /// This field has type `[Object; 1]` so that it will be correctly
    /// aligned. It's an array, rather than `Object`, because I
    /// *think* Rust's pointer-aliasing rules prohibit casting a
    /// pointer to an array element into a pointer to its containing
    /// array, but allow casting a pointer to an array into a shorter
    /// or longer array.
    body: [Object; 1],
}

impl<T: ?Sized> GcPtr<T> {
    #[cfg(target_arch = "aarch64")]
    #[inline(always)]
    fn obj_header(&self) -> &ObjHeader {
        unsafe { &*self.ptr}
    }
    #[cfg(not(target_arch = "aarch64"))]
    #[inline(always)]
    fn obj_header(&self) -> &ObjHeader {
        let ptr = tag::remove_tag(self.ptr);
        unsafe { &*ptr }
    }
    #[inline(always)]
    fn length(&self) -> Fixnum {
        self.obj_header().length
    }
    #[inline(always)]
    fn body(&self) -> *const [Object; 1] {
        &self.obj_header().body
    }
}

impl<T: ?Sized> Word for GcPtr<T> {
    #[inline(always)]
    fn to_u64(self) -> u64 { self.ptr.to_u64() }
    #[inline(always)]
    fn from_u64(u: u64) -> Self {
        debug_assert_eq!(tag::extract_tag(u), Tag::Gc);
        GcPtr {
            ptr: Word::from_u64(u),
            _t: PhantomData,
        }
    }
}

fn bytes_to_words(bytes: u64) -> u64 {
    (bytes + 7) / 8
}

fn size_in_words<T: Sized>() -> u64 {
    bytes_to_words(mem::size_of::<T>() as _)
}

impl<T: Sized> std::ops::Deref for GcPtr<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T {
        debug_assert!(mem::align_of::<T>() <= mem::align_of::<[Object; 1]>());
        debug_assert!(!self.ptr.is_null());
        debug_assert_eq!(size_in_words::<T>(), self.length().to_u64());
        let body = self.body() as *const T;
        unsafe { &*body }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::object::tag::*;
    #[test]
    fn top_byte_ignore() {
        for i in 0..=u8::MAX {
            let deadbeef = Box::new(0xdeadbeefu64);
            let deadbeef = Box::into_raw(deadbeef);
            let deadbeef = tag::add_tag(deadbeef, unsafe { std::mem::transmute(i) });
            assert_eq!(unsafe { *deadbeef }, 0xdeadbeefu64);
            let deadbeef = unsafe { Box::from_raw(tag::remove_tag(deadbeef)) };
            std::mem::drop(deadbeef);
        }
    }
}

