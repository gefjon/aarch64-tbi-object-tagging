use super::{Fixnum,
            Object,
            typeinfo::{Boxed, FixedSize, Type, VariableSize},
            word::Word,
            tag,
            Vector};
use std::{alloc::{AllocRef, Global, Layout},
          ops::Deref,
          mem,
          ptr};


#[repr(transparent)]
pub struct GcPtr<T> {
    ptr: *mut T,
}


impl<T: Boxed> GcPtr<T> {
    /// invariant: `ptr` must point to a valid instance of `T`
    /// allocated by the garbage collector
    unsafe fn from_raw(ptr: *mut T) -> Self { Self {
            ptr: tag::add_tag(ptr, T::ID),
    } }
}

impl<T: FixedSize> GcPtr<T> {
    pub fn alloc(t: T) -> GcPtr<T> {
        let layout = Layout::from_size_align(
            mem::size_of::<T>(),
            mem::align_of::<T>(),
        ).unwrap();
        let ptr = Global.alloc(layout).unwrap()
            .as_mut_ptr().cast::<T>();
        unsafe {
            ptr::write(ptr, t);
            Self::from_raw(ptr)
        }
    }
}

impl<T> GcPtr<Vector<T>>
where
    Vector<T>: VariableSize,
    T: Copy,
{
    pub fn alloc_vector(elts: &[T]) -> GcPtr<Vector<T>> {
        let layout = <Vector<T>>::layout(elts.len());
        let ptr = Global.alloc(layout).unwrap()
            .as_mut_ptr().cast::<Vector<T>>();
        unsafe {
            ptr::copy_nonoverlapping(
                elts.as_ptr(),
                <Vector<T>>::elementpointer_mut(ptr, Fixnum::from_u64(0)),
                elts.len(),
            );
            Self::from_raw(ptr)
        }
    }
}

impl<T: Boxed> Copy for GcPtr<T> {}
impl<T: Boxed> Clone for GcPtr<T> {
    fn clone(&self) -> Self { Self { ptr: self.ptr } }
}

impl<T: Boxed> GcPtr<T> {
    #[cfg(target_arch = "aarch64")]
    #[inline(always)]
    fn deref_inner(&self) -> &T {
        unsafe { &*self.ptr}
    }
    #[cfg(not(target_arch = "aarch64"))]
    #[inline(always)]
    fn deref_inner(&self) -> &T {
        let ptr = tag::remove_tag(self.ptr);
        unsafe { &*ptr }
    }
}

impl<T: Boxed> Word for GcPtr<T> {
    #[inline(always)]
    fn to_u64(self) -> u64 { self.ptr.to_u64() }
    #[inline(always)]
    fn from_u64(u: u64) -> Self {
        GcPtr {
            ptr: Word::from_u64(u),
        }
    }
}

#[allow(unused)]
fn is_aligned<T>(ptr: u64) -> bool {
    let ptr = ptr as usize;
    let align = mem::align_of::<T>();
    let mask = align - 1;
    (ptr & mask) == 0
}

impl<T: Boxed> Deref for GcPtr<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T {
        debug_assert!(is_aligned::<T>(self.to_u64()));
        debug_assert!(!self.ptr.is_null());
        debug_assert_eq!(tag::extract_tag(*self), T::ID);
        self.deref_inner()
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

