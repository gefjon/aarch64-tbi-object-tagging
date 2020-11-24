use super::typeinfo::TypeId;
use super::word::Word;

const TAG_SHIFT: u32 = 56;
const TAG_MASK: u64 = 0xff << TAG_SHIFT;


#[inline(always)]
pub fn extract_tag<W: Word>(word: W) -> TypeId {
    let word = word.to_u64();
    TypeId::from(((word & TAG_MASK) >> TAG_SHIFT) as u8)
}

#[inline(always)]
pub fn remove_tag<W: Word>(word: W) -> W {
    let word = word.to_u64();
    W::from_u64(word & !TAG_MASK)
}

#[inline(always)]
pub fn add_tag<W: Word>(word: W, tag: TypeId) -> W {
    let tag = (u8::from(tag) as u64) << TAG_SHIFT;
    let word = remove_tag(word);
    W::from_u64(tag | word.to_u64())
}
