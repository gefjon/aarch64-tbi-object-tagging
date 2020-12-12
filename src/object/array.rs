use super::{tag, Object, Fixnum, TypeId, Type, Boxed, Word};
use std::{alloc::Layout, borrow::Borrow, cmp::Ord, mem, ops::Index, ptr};

#[repr(C)]
pub struct Array<T> {
    length: Fixnum,
    /// note: there may be more than one `T` here, that is, we may
    /// effectively cast `&self.body` into a pointer to `[T;
    /// self.length]`.
    ///
    /// This field has type `[T; 1]` so that it will be correctly
    /// aligned. It's an array, rather than `T`, because I *think*
    /// Rust's pointer-aliasing rules prohibit casting a pointer to an
    /// array element into a pointer to its containing array, but
    /// allow casting a pointer to an array into a shorter or longer
    /// array.
    body: [T; 1],
}

unsafe impl<T> Boxed for Array<T>
where
    Array<T>: Type,
{}

unsafe impl Type for Array<Object> {
    const ID: TypeId = TypeId::ObjArray;
}

unsafe impl Type for Array<u8> {
    const ID: TypeId = TypeId::String;
}

impl<T> Array<T> {
    pub fn is_empty(&self) -> bool {
        self.len() == Fixnum::from_u64(0)
    }
    pub fn len(&self) -> Fixnum {
        self.length
    }
    /// returns an untagged pointer to the first element
    pub fn untagged_body_ptr(&self) -> *const T {
        tag::remove_tag(ptr::raw_const!(self.body) as *const T)
    }
    /// returns an untagged pointer to the `idx`th element
    ///
    /// unsafe for the same reason as (*const T)::add i.e. overflow is UB
    unsafe fn elementpointer(header: *const Self, idx: Fixnum) -> *const T {
        let idx = idx.to_u64() as usize;
        let base = tag::remove_tag(
            ptr::raw_const!((*header).body) as *const T
        );
        base.add(idx)
    }
    /// returns an untagged pointer to the `idx`th element
    ///
    /// unsafe for the same reason as (*const T)::add i.e. overflow is UB
    unsafe fn elementpointer_mut(header: *mut Self, idx: Fixnum) -> *mut T {
        let idx = idx.to_u64() as usize;
        let base = tag::remove_tag(
            ptr::raw_mut!((*header).body) as *mut T
        );
        base.add(idx)
    }
    /// invariant: `idx` must be in-bounds i.e. non-negative and less than `self.length`
    unsafe fn index_unchecked(&self, idx: Fixnum) -> &T {
        &*Self::elementpointer(self as *const Self, idx)
    }
    fn at(&self, idx: Fixnum) -> Option<&T> {
        if (idx < self.length) && (idx >= 0) {
            Some(unsafe { self.index_unchecked(idx) })
        } else {
            None
        }
    }
    pub fn layout(len: usize) -> Layout {
        let size_of_header = mem::size_of::<Self>();
        // call to `max` to avoid underflow
        let size_of_body = mem::size_of::<T>() * (Ord::max(len, 1) - 1);
        Layout::from_size_align(
            size_of_header + size_of_body,
            mem::align_of::<Self>(),
        ).unwrap()
    }
}

impl<T> Borrow<[T]> for Array<T> {
    fn borrow(&self) -> &[T] { unsafe {
        std::slice::from_raw_parts(self.untagged_body_ptr(), self.len().to_u64() as usize)
    } }
}

impl<T: Clone> Array<T> {
    /// initialize the `Array<T>` pointed to by `this` by cloning each
    /// of the `elements` into it
    ///
    /// # Safety
    /// `this` must point to a uniquely-allocated block of memory with
    /// space enough to hold an `Array<T>` with length
    /// `elements.len()`.
    pub unsafe fn initialize(this: *mut Self, elements: &[T]) {
        ptr::write(ptr::raw_mut!((*this).length), Fixnum::from_u64(elements.len() as _));
        for (i, elt) in elements.iter().enumerate() {
            ptr::write(
                Self::elementpointer_mut(this, Fixnum::from_u64(i as _)),
                elt.clone(),
            )
        }
    }
}

impl<T> Index<Fixnum> for Array<T> {
    type Output = T;
    fn index(&self, idx: Fixnum) -> &T {
        self.at(idx).unwrap()
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &T {
        self.at(Fixnum::from_u64(idx as _)).unwrap()
    }
}
