use super::Word;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TypeId {
    Fixnum = 0,
    ObjArray,
    String,
}

impl From<TypeId> for u8 {
    fn from(t: TypeId) -> u8 { t as _ }
}

impl From<u8> for TypeId {
    fn from(u: u8) -> TypeId {
        match u {
            0 => TypeId::Fixnum,
            1 => TypeId::ObjArray,
            2 => TypeId::String,
            _ => panic!("invalid tag {:x}", u),
        }
    }
}

/// # Safety
/// `ID` must be unique, that is, no other implementation of `Type`
/// may have the same `ID`
pub unsafe trait Type {
    const ID: TypeId;
}

/// # Safety
/// must be disjoint from `Boxed`; must be encoded inline within a
/// tagged `Object`
pub unsafe trait Immediate: Type + Word {}

/// # Safety
/// must be disjoint from `Immediate`; must be encoded into an
/// `Object` as a `GcPtr<Self>`
pub unsafe trait Boxed: Type {}

/// Analogous to `Sized`, to fool rustc.
///
/// Denotes objects for which `mem::size_of::<Self>()` is always equal
/// to the allocated object's size.  Any object which is `Boxed` but
/// not `FixedSize` will not meet that invariant.
///
/// # Safety
/// Allocated instances of these types must always be of size
/// `mem::size_of::<Self>()`
pub unsafe trait FixedSize: Boxed {}
