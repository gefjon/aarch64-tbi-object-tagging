const TAG_MASK: u64 = 0xff00_0000_0000_0000;
const TAG_SHIFT: u32 = 56;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Tag {
    Fixnum = 0,
    Gc = 1,
}

impl From<Tag> for u8 {
    fn from(t: Tag) -> u8 { t as _ }
}

impl From<u8> for Tag {
    fn from(u: u8) -> Tag {
        match u {
            0 => Tag::Fixnum,
            1 => Tag::Gc,
            _ => panic!("invalid tag {:x}", u),
        }
    }
}

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

impl<T> Word for *mut T {
    #[inline(always)]
    fn to_u64(self) -> u64 { self as _ }
    #[inline(always)]
    fn from_u64(u: u64) -> Self { u as _ }
}

#[inline(always)]
pub fn extract_tag<W: Word>(word: W) -> Tag {
    let word = word.to_u64();
    Tag::from(((word & TAG_MASK) >> TAG_SHIFT) as u8)
}

#[inline(always)]
pub fn remove_tag<W: Word>(word: W) -> W {
    let word = word.to_u64();
    W::from_u64(word & !TAG_MASK)
}

#[inline(always)]
pub fn add_tag<W: Word>(word: W, tag: Tag) -> W {
    let tag = (u8::from(tag) as u64) << TAG_SHIFT;
    let word = remove_tag(word);
    W::from_u64(tag | word.to_u64())
}
