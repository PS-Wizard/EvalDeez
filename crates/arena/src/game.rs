#![allow(dead_code)]

use crate::{
    board::Board,
    piece::{Color, Piece, PieceType},
};

pub struct Game {
    pub white_pawns: Board,
    pub white_rooks: Board,
    pub white_knights: Board,
    pub white_bishops: Board,
    pub white_queens: Board,
    pub white_king: Board,

    pub black_pawns: Board,
    pub black_rooks: Board,
    pub black_knights: Board,
    pub black_bishops: Board,
    pub black_queens: Board,
    pub black_king: Board,

    pub castling_rights: u8, // 4 bits: WK, WQ, BK, BQ
    pub side_to_move: bool,  // false = white, true = black

                             //TODO:
                             // - Enpassant flag
                             // Move Counters
}

impl Game {
    pub fn new() -> Self {
        Game {
            // White pieces on ranks 1 and 2
            white_pawns: Board(0x0000_0000_0000_FF00),
            white_knights: Board(0x0000_0000_0000_0042),
            white_bishops: Board(0x0000_0000_0000_0024),
            white_rooks: Board(0x0000_0000_0000_0081),
            white_queens: Board(0x0000_0000_0000_0008),
            white_king: Board(0x0000_0000_0000_0010),

            // Black pieces on ranks 7 and 8
            black_pawns: Board(0x00FF_0000_0000_0000),
            black_knights: Board(0x4200_0000_0000_0000),
            black_bishops: Board(0x2400_0000_0000_0000),
            black_rooks: Board(0x8100_0000_0000_0000),
            black_queens: Board(0x0800_0000_0000_0000),
            black_king: Board(0x1000_0000_0000_0000),

            castling_rights: 0,
            side_to_move: true,
        }
    }

    pub fn all_pieces(&self) -> Board {
        Board(
            self.white_pawns.0
                | self.white_knights.0
                | self.white_bishops.0
                | self.white_rooks.0
                | self.white_queens.0
                | self.white_king.0
                | self.black_pawns.0
                | self.black_knights.0
                | self.black_bishops.0
                | self.black_rooks.0
                | self.black_queens.0
                | self.black_king.0,
        )
    }

    pub fn get_piece_at(&self, idx: u8) -> Option<Piece> {
        if idx >= 64 || (self.all_pieces().0 & 1 << idx == 0) {
            return None;
        }

        // Helper closure to check if bit is set and create Piece
        let check = |board: &Board, piece_type: PieceType, color: Color| {
            if board.has_bit(idx) {
                Some(Piece::new(piece_type, color))
            } else {
                None
            }
        };

        // Check white pieces
        if let Some(p) = check(&self.white_pawns, PieceType::Pawn, Color::White) {
            return Some(p);
        }
        if let Some(p) = check(&self.white_knights, PieceType::Knight, Color::White) {
            return Some(p);
        }
        if let Some(p) = check(&self.white_bishops, PieceType::Bishop, Color::White) {
            return Some(p);
        }
        if let Some(p) = check(&self.white_rooks, PieceType::Rook, Color::White) {
            return Some(p);
        }
        if let Some(p) = check(&self.white_queens, PieceType::Queen, Color::White) {
            return Some(p);
        }
        if let Some(p) = check(&self.white_king, PieceType::King, Color::White) {
            return Some(p);
        }

        // Check black pieces
        if let Some(p) = check(&self.black_pawns, PieceType::Pawn, Color::Black) {
            return Some(p);
        }
        if let Some(p) = check(&self.black_knights, PieceType::Knight, Color::Black) {
            return Some(p);
        }
        if let Some(p) = check(&self.black_bishops, PieceType::Bishop, Color::Black) {
            return Some(p);
        }
        if let Some(p) = check(&self.black_rooks, PieceType::Rook, Color::Black) {
            return Some(p);
        }
        if let Some(p) = check(&self.black_queens, PieceType::Queen, Color::Black) {
            return Some(p);
        }
        if let Some(p) = check(&self.black_king, PieceType::King, Color::Black) {
            return Some(p);
        }

        None
    }
}

#[cfg(test)]
mod game_test {
    use super::*;
    use crate::piece::{Color, PieceType};

    #[test]
    fn test_initial_positions() {
        let game = Game::new();

        // White pawns on rank 2 (index 8..15)
        for i in 8..16 {
            let piece = game.get_piece_at(i).expect("Expected a piece");
            assert_eq!(piece.piece_type(), PieceType::Pawn);
            assert_eq!(piece.color(), Color::White);
        }

        // White king on e1 (index 4)
        let king = game.get_piece_at(4).expect("Expected white king");
        assert_eq!(king.piece_type(), PieceType::King);
        assert_eq!(king.color(), Color::White);

        // Black pawns on rank 7 (index 48..55)
        for i in 48..56 {
            let piece = game.get_piece_at(i).expect("Expected a piece");
            assert_eq!(piece.piece_type(), PieceType::Pawn);
            assert_eq!(piece.color(), Color::Black);
        }

        // Black king on e8 (index 60)
        let black_king = game.get_piece_at(60).expect("Expected black king");
        assert_eq!(black_king.piece_type(), PieceType::King);
        assert_eq!(black_king.color(), Color::Black);

        // Empty square: a4 (index 24)
        assert!(game.get_piece_at(24).is_none());
    }
}
