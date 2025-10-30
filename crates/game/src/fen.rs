use types::rights::CastleRights;
use types::{
    bitboard::Bitboard,
    piece_type::{Color, Piece},
};

use crate::position::{ColoredPiece, Position};

impl Position {
    /// Parses a FEN string into a Position
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.trim().split_whitespace().collect();
        if parts.len() != 6 {
            return Err("FEN string must have exactly 6 parts".to_string());
        }

        let mut position = Position {
            pieces: [Bitboard::new(); 6],
            colors: [Bitboard::new(); 2],
            piece_map: [None; 64],
            side_to_move: Self::parse_side_to_move(parts[1])?,
            castling_rights: Self::parse_castling_rights(parts[2])?,
            en_passant: Self::parse_en_passant(parts[3])?,
            halfmove_clock: parts[4].parse().map_err(|_| "Invalid halfmove clock")?,
            fullmove_clock: parts[5].parse().map_err(|_| "Invalid fullmove number")?,
            hash: 0,
        };

        Self::parse_piece_placement(&mut position, parts[0])?;
        Ok(position)
    }

    fn parse_piece_placement(position: &mut Position, placement: &str) -> Result<(), String> {
        let ranks: Vec<&str> = placement.split('/').collect();
        if ranks.len() != 8 {
            return Err("Piece placement must have 8 ranks".to_string());
        }

        for (rank_idx, rank) in ranks.iter().enumerate() {
            let mut file_idx = 0;

            for ch in rank.chars() {
                if file_idx >= 8 {
                    return Err("Too many pieces/squares in rank".to_string());
                }

                if let Some(skip) = ch.to_digit(10) {
                    file_idx += skip as usize;
                    if file_idx > 8 {
                        return Err("Invalid empty square count".to_string());
                    }
                } else {
                    let colored_piece = Self::char_to_piece(ch)?;
                    let sq = (7 - rank_idx) * 8 + file_idx;

                    position.piece_map[sq] = Some(colored_piece);
                    position.pieces[colored_piece.piece()].set_bit(sq);
                    position.colors[colored_piece.color()].set_bit(sq);

                    file_idx += 1;
                }
            }

            if file_idx != 8 {
                return Err("Rank doesn't have exactly 8 squares".to_string());
            }
        }

        Ok(())
    }

    #[inline(always)]
    fn char_to_piece(ch: char) -> Result<ColoredPiece, String> {
        let (piece, color) = match ch {
            'P' => (Piece::Pawn, Color::White),
            'N' => (Piece::Knight, Color::White),
            'B' => (Piece::Bishop, Color::White),
            'R' => (Piece::Rook, Color::White),
            'Q' => (Piece::Queen, Color::White),
            'K' => (Piece::King, Color::White),
            'p' => (Piece::Pawn, Color::Black),
            'n' => (Piece::Knight, Color::Black),
            'b' => (Piece::Bishop, Color::Black),
            'r' => (Piece::Rook, Color::Black),
            'q' => (Piece::Queen, Color::Black),
            'k' => (Piece::King, Color::Black),
            _ => return Err(format!("Invalid piece character: {}", ch)),
        };
        Ok(ColoredPiece::new(piece, color))
    }

    #[inline(always)]
    fn parse_side_to_move(s: &str) -> Result<Color, String> {
        match s {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err("Side to move must be 'w' or 'b'".to_string()),
        }
    }

    #[inline(always)]
    fn parse_castling_rights(s: &str) -> Result<CastleRights, String> {
        if s == "-" {
            return Ok(CastleRights::NONE);
        }

        let mut rights = 0u8;
        for ch in s.chars() {
            rights |= match ch {
                'K' => CastleRights::WHITE_KING.0,
                'Q' => CastleRights::WHITE_QUEEN.0,
                'k' => CastleRights::BLACK_KING.0,
                'q' => CastleRights::BLACK_QUEEN.0,
                _ => return Err(format!("Invalid castling right: {}", ch)),
            };
        }
        Ok(CastleRights(rights))
    }

    #[inline(always)]
    fn parse_en_passant(s: &str) -> Result<Option<u8>, String> {
        if s == "-" {
            return Ok(None);
        }

        let bytes = s.as_bytes();
        if bytes.len() != 2 {
            return Err("En passant square must be 2 characters".to_string());
        }

        let file = bytes[0];
        let rank = bytes[1];

        if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
            return Err("Invalid en passant square".to_string());
        }

        Ok(Some((rank - b'1') * 8 + (file - b'a')))
    }
}

#[cfg(test)]
mod fen {
    use super::*;
    use utilities::{algebraic::Algebraic, board::PrintAsBoard};

    #[test]
    fn test_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let position = Position::from_fen(fen).unwrap();

        // Test that white king is on e1
        assert_eq!(
            position.piece_map["e1".idx()],
            Some(ColoredPiece::new(Piece::King, Color::White))
        );

        // Test black King on e8
        assert_eq!(
            position.piece_map["e8".idx()],
            Some(ColoredPiece::new(Piece::King, Color::Black))
        );

        // Test that white rook is on a1
        assert_eq!(
            position.piece_map["a1".idx()],
            Some(ColoredPiece::new(Piece::Rook, Color::White))
        );

        // Test that black rook is on a8
        assert_eq!(
            position.piece_map["a8".idx()],
            Some(ColoredPiece::new(Piece::Rook, Color::Black))
        );

        // Test side to move
        assert_eq!(position.side_to_move, Color::White);

        // Test castling rights (all available)
        assert_eq!(position.castling_rights.0, 0b1111);

        // Test no en passant
        assert_eq!(position.en_passant, None);

        // Test halfmove and fullmove counters
        assert_eq!(position.halfmove_clock, 0);
        assert_eq!(position.fullmove_clock, 1);

        // Test Verify Visually
        for board in position.pieces {
            board.0.print();
        }
    }
}
