use crate::position::Position;
use types::move_type::{Move, MoveType::*};
use types::piece_type::{
    Color::{self, *},
    Piece::{self, *},
};
use types::rights::CastleRights;

/// Stores all information needed to undo a move
#[derive(Clone, Copy, Debug)]
pub struct UndoInfo {
    pub captured_piece: Option<Piece>,
    pub captured_color: Option<Color>,
    pub castling_rights: CastleRights,
    pub en_passant: Option<u8>,
    pub halfmove_clock: u8,

    // Store the captured square for en passant (different from move.to())
    pub ep_capture_square: Option<usize>,
}

impl Position {
    /// Makes a move in place and returns undo information
    pub fn make_move(&mut self, m: Move) -> UndoInfo {
        let from = m.from();
        let to = m.to();
        let move_type = m.move_type();

        // Save state for undo
        let mut undo = UndoInfo {
            captured_piece: None,
            captured_color: None,
            castling_rights: self.castling_rights,
            en_passant: self.en_passant,
            halfmove_clock: self.halfmove_clock,
            ep_capture_square: None,
        };

        // Get the moving piece
        let moving_piece_info = self.piece_at(from).expect("No piece at from square");
        let moving_piece = moving_piece_info.piece();
        let moving_color = moving_piece_info.color();

        match move_type {
            Quiet => {
                self.remove_piece(from);
                self.add_piece(to, moving_piece, moving_color);
            }
            Capture => {
                if let Some(captured) = self.piece_at(to) {
                    undo.captured_piece = Some(captured.piece());
                    undo.captured_color = Some(captured.color());
                }
                self.remove_piece(to);
                self.remove_piece(from);
                self.add_piece(to, moving_piece, moving_color);
            }
            DoublePush => {
                self.remove_piece(from);
                self.add_piece(to, moving_piece, moving_color);
                // Set en passant square
                let ep_sq = if self.side_to_move == White {
                    from + 8
                } else {
                    from - 8
                };
                self.en_passant = Some(ep_sq as u8);
            }
            EnPassant => {
                let captured_sq = if self.side_to_move == White {
                    to - 8
                } else {
                    to + 8
                };
                if let Some(captured) = self.piece_at(captured_sq) {
                    undo.captured_piece = Some(captured.piece());
                    undo.captured_color = Some(captured.color());
                }
                undo.ep_capture_square = Some(captured_sq);

                self.remove_piece(captured_sq);
                self.remove_piece(from);
                self.add_piece(to, moving_piece, moving_color);
            }
            Castle => {
                // Move king
                self.remove_piece(from);
                self.add_piece(to, moving_piece, moving_color);

                // Move rook
                let (rook_from, rook_to) = match to {
                    6 => (7, 5),    // White kingside
                    2 => (0, 3),    // White queenside
                    62 => (63, 61), // Black kingside
                    58 => (56, 59), // Black queenside
                    _ => panic!("Invalid castle destination: {}", to),
                };
                self.remove_piece(rook_from);
                self.add_piece(rook_to, Rook, moving_color);
            }
            PromotionQueen | PromotionRook | PromotionBishop | PromotionKnight => {
                self.remove_piece(from);
                let promoted = self.get_promotion_piece(move_type);
                self.add_piece(to, promoted, moving_color);
            }
            CapturePromotionQueen
            | CapturePromotionRook
            | CapturePromotionBishop
            | CapturePromotionKnight => {
                if let Some(captured) = self.piece_at(to) {
                    undo.captured_piece = Some(captured.piece());
                    undo.captured_color = Some(captured.color());
                }
                self.remove_piece(to);
                self.remove_piece(from);
                let promoted = self.get_promotion_piece(move_type);
                self.add_piece(to, promoted, moving_color);
            }
        }

        // Update castling rights
        self.update_castling_rights(from, to, moving_piece, moving_color);

        // Clear en passant if not a double push
        if move_type != DoublePush {
            self.en_passant = None;
        }

        // Switch turns
        self.side_to_move = self.side_to_move.flip();

        // Update clocks
        if matches!(moving_piece, Pawn)
            || matches!(
                move_type,
                Capture
                    | CapturePromotionQueen
                    | CapturePromotionRook
                    | CapturePromotionBishop
                    | CapturePromotionKnight
                    | EnPassant
            )
        {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        if self.side_to_move == White {
            self.fullmove_clock += 1;
        }

        undo
    }

    /// Unmakes a move using the undo information
    pub fn unmake_move(&mut self, m: Move, undo: UndoInfo) {
        let from = m.from();
        let to = m.to();
        let move_type = m.move_type();

        // Switch turns back
        self.side_to_move = self.side_to_move.flip();

        // Restore clocks
        self.halfmove_clock = undo.halfmove_clock;
        if self.side_to_move == Black {
            self.fullmove_clock -= 1;
        }

        let moving_color = self.side_to_move;

        match move_type {
            Quiet => {
                let piece_info = self.piece_at(to).expect("No piece at destination");
                let moving_piece = piece_info.piece();
                self.remove_piece(to);
                self.add_piece(from, moving_piece, moving_color);
            }
            Capture => {
                let piece_info = self.piece_at(to).expect("No piece at destination");
                let moving_piece = piece_info.piece();
                self.remove_piece(to);
                self.add_piece(from, moving_piece, moving_color);
                if let (Some(piece), Some(color)) = (undo.captured_piece, undo.captured_color) {
                    self.add_piece(to, piece, color);
                }
            }
            DoublePush => {
                let piece_info = self.piece_at(to).expect("No piece at destination");
                let moving_piece = piece_info.piece();
                self.remove_piece(to);
                self.add_piece(from, moving_piece, moving_color);
            }
            EnPassant => {
                self.remove_piece(to);
                self.add_piece(from, Pawn, moving_color);

                if let Some(captured_sq) = undo.ep_capture_square {
                    if let (Some(piece), Some(color)) = (undo.captured_piece, undo.captured_color) {
                        self.add_piece(captured_sq, piece, color);
                    }
                }
            }
            Castle => {
                // Unmove king
                self.remove_piece(to);
                self.add_piece(from, King, moving_color);

                // Unmove rook
                let (rook_from, rook_to) = match to {
                    6 => (7, 5),    // White kingside
                    2 => (0, 3),    // White queenside
                    62 => (63, 61), // Black kingside
                    58 => (56, 59), // Black queenside
                    _ => panic!("Invalid castle destination: {}", to),
                };
                self.remove_piece(rook_to);
                self.add_piece(rook_from, Rook, moving_color);
            }
            PromotionQueen | PromotionRook | PromotionBishop | PromotionKnight => {
                self.remove_piece(to);
                self.add_piece(from, Pawn, moving_color);
            }
            CapturePromotionQueen
            | CapturePromotionRook
            | CapturePromotionBishop
            | CapturePromotionKnight => {
                self.remove_piece(to);
                self.add_piece(from, Pawn, moving_color);
                if let (Some(piece), Some(color)) = (undo.captured_piece, undo.captured_color) {
                    self.add_piece(to, piece, color);
                }
            }
        }

        // Restore state
        self.castling_rights = undo.castling_rights;
        self.en_passant = undo.en_passant;
    }

    /// Helper function to get the promotion piece depending on the flag
    fn get_promotion_piece(
        &self,
        move_type: types::move_type::MoveType,
    ) -> types::piece_type::Piece {
        match move_type {
            PromotionQueen | CapturePromotionQueen => Queen,
            PromotionRook | CapturePromotionRook => Rook,
            PromotionBishop | CapturePromotionBishop => Bishop,
            PromotionKnight | CapturePromotionKnight => Knight,
            _ => panic!("Not a promotion move"),
        }
    }

    /// Updates castling rights depending on if the king moved or the rooks did
    fn update_castling_rights(
        &mut self,
        from: usize,
        to: usize,
        piece: types::piece_type::Piece,
        color: Color,
    ) {
        // King moves lose all castling rights for that color
        if piece == King {
            if color == White {
                self.castling_rights.remove_castling(White);
            } else {
                self.castling_rights.remove_castling(Black);
            }
        }

        // Rook moves or captures lose specific castling rights
        match from {
            0 => self.castling_rights.remove_queenside(White),
            7 => self.castling_rights.remove_kingside(White),
            56 => self.castling_rights.remove_queenside(Black),
            63 => self.castling_rights.remove_kingside(Black),
            _ => {}
        }

        match to {
            0 => self.castling_rights.remove_queenside(White),
            7 => self.castling_rights.remove_kingside(White),
            56 => self.castling_rights.remove_queenside(Black),
            63 => self.castling_rights.remove_kingside(Black),
            _ => {}
        }
    }
}

#[cfg(test)]
mod make_unmake_tests {
    use crate::position::Position;
    use types::move_collector::MoveCollector;

    #[test]
    fn test_make_unmake_reversible() {
        let positions = [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        ];

        for fen in positions {
            let mut pos = Position::from_fen(fen).unwrap();
            let original = pos.clone();

            let mut collector = MoveCollector::new();
            pos.generate_pseudo_legal_moves(&mut collector);

            for m in collector.captures_iter() {
                let undo = pos.make_move(m);
                pos.unmake_move(m, undo);

                assert_eq!(
                    pos, original,
                    "Position not restored after make/unmake for move {:?}",
                    m
                );
            }

            for m in collector.quiets_iter() {
                let undo = pos.make_move(m);
                pos.unmake_move(m, undo);

                assert_eq!(
                    pos, original,
                    "Position not restored after make/unmake for move {:?}",
                    m
                );
            }
        }
    }

    #[test]
    fn test_castle_make_unmake() {
        // White kingside castle
        let mut pos =
            Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1").unwrap();
        let original = pos.clone();

        let mut collector = MoveCollector::new();
        pos.generate_pseudo_legal_moves(&mut collector);

        // Find the castle move
        for m in collector.quiets_iter() {
            if m.move_type() == types::move_type::MoveType::Castle {
                let undo = pos.make_move(m);
                pos.unmake_move(m, undo);
                assert_eq!(pos, original);
            }
        }
    }

    #[test]
    fn test_en_passant_make_unmake() {
        let mut pos =
            Position::from_fen("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 1")
                .unwrap();
        let original = pos.clone();

        let mut collector = MoveCollector::new();
        pos.generate_pseudo_legal_moves(&mut collector);

        for m in collector.captures_iter() {
            if m.move_type() == types::move_type::MoveType::EnPassant {
                let undo = pos.make_move(m);
                pos.unmake_move(m, undo);
                assert_eq!(pos, original);
            }
        }
    }

    #[test]
    fn test_promotion_make_unmake() {
        let mut pos =
            Position::from_fen("rnbqkbnr/ppppppPp/8/8/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 1").unwrap();
        let original = pos.clone();

        let mut collector = MoveCollector::new();
        pos.generate_pseudo_legal_moves(&mut collector);

        for m in collector.quiets_iter() {
            let undo = pos.make_move(m);
            pos.unmake_move(m, undo);
            assert_eq!(pos, original);
        }
    }
}
