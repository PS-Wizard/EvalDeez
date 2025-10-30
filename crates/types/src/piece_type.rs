use std::ops::{Index, IndexMut};

/// White / Black Color Enum, maps to the same order stored in the side, in `Position` struct
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    #[inline(always)]
    pub const fn flip(self) -> Color {
        unsafe { std::mem::transmute((self as u8) ^ 1) }
    }

    #[inline(always)]
    pub const fn index(self) -> usize {
        self as usize
    }
}

impl<T> Index<Color> for [T; 2] {
    type Output = T;

    #[inline(always)]
    fn index(&self, color: Color) -> &Self::Output {
        &self[color as usize]
    }
}

impl<T> IndexMut<Color> for [T; 2] {
    #[inline(always)]
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        &mut self[color as usize]
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl Piece {
    #[inline(always)]
    pub const fn index(self) -> usize {
        self as usize
    }
}

impl<T> Index<Piece> for [T; 6] {
    type Output = T;

    #[inline(always)]
    fn index(&self, piece: Piece) -> &Self::Output {
        &self[piece as usize]
    }
}

impl<T> IndexMut<Piece> for [T; 6] {
    #[inline(always)]
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        &mut self[piece as usize]
    }
}
