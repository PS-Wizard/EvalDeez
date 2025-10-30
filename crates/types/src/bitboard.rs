use std::ops::{BitAnd, BitOr, BitOrAssign};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    #[inline(always)]
    pub const fn new() -> Self {
        Bitboard(0)
    }

    #[inline(always)]
    pub const fn from_raw(value: u64) -> Self {
        Bitboard(value)
    }

    #[inline(always)]
    pub fn set_bit(&mut self, idx: usize) {
        self.0 |= 1 << idx;
    }

    #[inline(always)]
    pub fn remove_bit(&mut self, idx: usize) {
        self.0 &= !(1 << idx);
    }

    #[inline(always)]
    pub fn pop_lsb(&mut self) -> usize {
        let idx = self.0.trailing_zeros() as usize;
        self.0 &= self.0 - 1; // This is faster than self.0 &= !(1 << idx)
        idx
    }

    #[inline(always)]
    pub const fn count(self) -> u32 {
        self.0.count_ones()
    }

    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.0 != 0
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}
