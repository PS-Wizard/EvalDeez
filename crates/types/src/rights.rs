use crate::piece_type::Color;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CastleRights(pub u8);
impl CastleRights {
    pub const NONE: Self = CastleRights(0);
    pub const WHITE_KING: Self = CastleRights(1);
    pub const WHITE_QUEEN: Self = CastleRights(2);
    pub const BLACK_KING: Self = CastleRights(4);
    pub const BLACK_QUEEN: Self = CastleRights(8);
    pub const ALL: Self = CastleRights(15);

    #[inline(always)]
    pub const fn can_castle_kingside(self, color: Color) -> bool {
        let mask = if color as u8 == Color::White as u8 {
            Self::WHITE_KING.0
        } else {
            Self::BLACK_KING.0
        };
        self.0 & mask != 0
    }

    #[inline(always)]
    pub const fn can_castle_queenside(self, color: Color) -> bool {
        let mask = if color as u8 == Color::White as u8 {
            Self::WHITE_QUEEN.0
        } else {
            Self::BLACK_QUEEN.0
        };
        self.0 & mask != 0
    }

    #[inline(always)]
    pub fn remove_castling(&mut self, color: Color) {
        // OPTIMIZATION: Single operation instead of two methods
        let mask = match color {
            Color::White => Self::WHITE_KING.0 | Self::WHITE_QUEEN.0,
            Color::Black => Self::BLACK_KING.0 | Self::BLACK_QUEEN.0,
        };
        self.0 &= !mask;
    }

    #[inline(always)]
    pub fn remove_kingside(&mut self, color: Color) {
        let mask = match color {
            Color::White => Self::WHITE_KING.0,
            Color::Black => Self::BLACK_KING.0,
        };
        self.0 &= !mask;
    }

    #[inline(always)]
    pub fn remove_queenside(&mut self, color: Color) {
        let mask = match color {
            Color::White => Self::WHITE_QUEEN.0,
            Color::Black => Self::BLACK_QUEEN.0,
        };
        self.0 &= !mask;
    }
}
