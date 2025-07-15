#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece(u8);

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self((piece_type as u8) | ((color as u8) << 3))
    }

    pub fn piece_type(self) -> PieceType {
        match self.0 & 0b111 {
            0 => PieceType::Pawn,
            1 => PieceType::Knight,
            2 => PieceType::Bishop,
            3 => PieceType::Rook,
            4 => PieceType::Queen,
            5 => PieceType::King,
            _ => unreachable!(),
        }
    }

    pub fn color(self) -> Color {
        if (self.0 & 0b1000) != 0 {
            Color::Black
        } else {
            Color::White
        }
    }

    pub fn is_white(self) -> bool {
        self.color() == Color::White
    }

    pub fn is_black(self) -> bool {
        self.color() == Color::Black
    }
}

impl std::fmt::Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Piece")
            .field("piece_type", &self.piece_type())
            .field("color", &self.color())
            .finish()
    }
}

#[cfg(test)]
mod piece_test {
    use super::*;

    #[test]
    fn test_piece_new_and_accessors() {
        let p = Piece::new(PieceType::Queen, Color::Black);
        assert_eq!(p.piece_type(), PieceType::Queen);
        assert_eq!(p.color(), Color::Black);
        assert!(p.is_black());
        assert!(!p.is_white());

        let p2 = Piece::new(PieceType::Pawn, Color::White);
        assert_eq!(p2.piece_type(), PieceType::Pawn);
        assert_eq!(p2.color(), Color::White);
        assert!(p2.is_white());
        assert!(!p2.is_black());
    }

    #[test]
    fn test_piece_debug_format() {
        let p = Piece::new(PieceType::Rook, Color::White);
        let debug_str = format!("{:?}", p);
        println!("{:#?}", p);
        assert!(debug_str.contains("piece_type: Rook"));
        assert!(debug_str.contains("color: White"));

        let p_black = Piece::new(PieceType::King, Color::Black);
        let debug_str_black = format!("{:?}", p_black);
        assert!(debug_str_black.contains("piece_type: King"));
        assert!(debug_str_black.contains("color: Black"));
    }

    #[test]
    #[should_panic(expected = "unreachable")]
    fn test_piece_type_unreachable() {
        // Manually create invalid Piece with invalid piece type bits to test unreachable!
        let invalid_piece = Piece(0b111); // 7 is invalid piece type
        let _ = invalid_piece.piece_type(); // should panic
    }
}
