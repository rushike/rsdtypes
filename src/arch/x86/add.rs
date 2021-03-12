use crate::arch::Word;

/// Add a + b + carry.
///
/// Returns (result, overflow).
pub(crate) fn add_with_carry(a: Word, b: Word, carry: bool) -> (Word, bool) {
    let mut sum = 0;
    let carry = unsafe { core::arch::x86::_addcarry_u32(carry.into(), a, b, &mut sum) };
    (sum, carry != 0)
}

/// Subtract a - b - borrow.
///
/// Returns (result, overflow).
pub(crate) fn sub_with_borrow(a: Word, b: Word, borrow: bool) -> (Word, bool) {
    const_assert!(WORD_BITS == 32);
    let mut diff = 0;
    let borrow = unsafe { core::arch::x86::_subborrow_u32(borrow.into(), a, b, &mut diff) };
    (diff, borrow != 0)
}