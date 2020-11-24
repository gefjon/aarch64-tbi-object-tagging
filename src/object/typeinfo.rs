#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TypeId {
    Fixnum = 0,
    ObjVector,
    String,
    NegFixnum = 0xff,
}

impl From<TypeId> for u8 {
    fn from(t: TypeId) -> u8 { t as _ }
}

impl From<u8> for TypeId {
    fn from(u: u8) -> TypeId {
        match u {
            0 => TypeId::Fixnum,
            1 => TypeId::ObjVector,
            2 => TypeId::String,
            _ => panic!("invalid tag {:x}", u),
        }
    }
}

/// invariant: `ID` must be unique, that is, no other implementation
/// of `Type` may have the smae `ID`
pub unsafe trait Type {
    const ID: TypeId;
}

/// invariant: must be disjoint from `Boxed`
pub unsafe trait Immediate: Type {}

/// invariant: must be disjoint from `Immediate`
pub unsafe trait Boxed: Type {}

/// invariant: must be disjoint from `VariableSize`
pub unsafe trait FixedSize: Boxed {}

/// invariant: must be disjoint from `FixedSize`
pub unsafe trait VariableSize: Boxed {}
