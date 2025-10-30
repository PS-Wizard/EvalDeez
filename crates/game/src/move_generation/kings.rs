use crate::position::Position;
use raw::KING_ATTACKS;
use types::move_collector::MoveCollector;
use types::move_type::{Move, MoveType::*};
use types::piece_type::{Color::*, Piece::*};

impl Position {
    #[inline(always)]
    /// Generates all pseudo-legal king moves
    /// NOTE: Generates **legal** castling moves, other king moves are pseudo-legal
    pub fn generate_king_moves(&self, collector: &mut MoveCollector) {
        let king_sq = self.friendly(King).0.trailing_zeros() as usize;
        let friendly = self.us().0;
        let enemy = self.them().0;

        // Get all squares the king could potentially move to
        let potential_moves = KING_ATTACKS[king_sq] & !friendly;

        // Generate captures
        let mut capture_bb = potential_moves & enemy;
        while capture_bb != 0 {
            let to = capture_bb.trailing_zeros() as usize;
            capture_bb &= capture_bb - 1;
            collector.push_capture(Move::new(king_sq, to, Capture));
        }

        // Generate quiet moves
        let mut quiet_bb = potential_moves & !enemy;
        while quiet_bb != 0 {
            let to = quiet_bb.trailing_zeros() as usize;
            quiet_bb &= quiet_bb - 1;
            collector.push_quiet(Move::new(king_sq, to, Quiet));
        }

        // Castling - generate pseudo-legal castling moves
        self.generate_castling_moves(collector);
    }

    /// Generates legal castling moves - checks all castling requirements
    /// Castling is legal only if:
    /// 1. King has castling rights
    /// 2. Squares between king and rook are empty
    /// 3. King is not in check
    /// 4. King doesn't pass through check
    /// 5. King doesn't land in check
    fn generate_castling_moves(&self, collector: &mut MoveCollector) {
        // Can't castle if in check
        if self.is_in_check() {
            return;
        }

        let all_pieces = self.colors[0].0 | self.colors[1].0;
        let enemy_color = self.side_to_move.flip();

        if self.side_to_move == White {
            // White kingside: e1-g1
            if self.castling_rights.can_castle_kingside(White) {
                // f1 and g1 must be empty (0x60 = bits 5 and 6)
                if (all_pieces & 0x60) == 0 {
                    // f1 and g1 must not be attacked
                    if !self.is_square_attacked(5, enemy_color, None)
                        && !self.is_square_attacked(6, enemy_color, None)
                    {
                        collector.push_quiet(Move::new(4, 6, Castle));
                    }
                }
            }

            // White queenside: e1-c1
            if self.castling_rights.can_castle_queenside(White) {
                // b1, c1, d1 must be empty (0x0E = bits 1, 2, 3)
                if (all_pieces & 0x0E) == 0 {
                    // c1 and d1 must not be attacked (b1 doesn't matter)
                    if !self.is_square_attacked(2, enemy_color, None)
                        && !self.is_square_attacked(3, enemy_color, None)
                    {
                        collector.push_quiet(Move::new(4, 2, Castle));
                    }
                }
            }
        } else {
            // Black kingside: e8-g8
            if self.castling_rights.can_castle_kingside(Black) {
                if (all_pieces & 0x6000000000000000) == 0 {
                    // f8 and g8 must not be attacked
                    if !self.is_square_attacked(61, enemy_color, None)
                        && !self.is_square_attacked(62, enemy_color, None)
                    {
                        collector.push_quiet(Move::new(60, 62, Castle));
                    }
                }
            }

            // Black queenside: e8-c8
            if self.castling_rights.can_castle_queenside(Black) {
                if (all_pieces & 0x0E00000000000000) == 0 {
                    // c8 and d8 must not be attacked (b8 doesn't matter)
                    if !self.is_square_attacked(58, enemy_color, None)
                        && !self.is_square_attacked(59, enemy_color, None)
                    {
                        collector.push_quiet(Move::new(60, 58, Castle));
                    }
                }
            }
        }
    }

    #[inline(always)]
    /// Checks if the current side to move's king is in check
    pub fn is_in_check(&self) -> bool {
        let king_sq = self.friendly(King).0.trailing_zeros() as usize;
        self.is_square_attacked(king_sq, self.side_to_move.flip(), None)
    }

    #[inline(always)]
    /// Checks if the enemy's king is in check
    pub fn is_enemy_in_check(&self) -> bool {
        let king_sq = self.enemy(King).0.trailing_zeros() as usize;
        self.is_square_attacked(king_sq, self.side_to_move, None)
    }
}

#[cfg(test)]
mod kings {
    use crate::position::Position;
    use types::move_collector::MoveCollector;

    #[test]
    fn test_king_pseudo_legal() {
        // Initial position expected 0 king moves
        let g = Position::new();
        let mut mc = MoveCollector::new();
        g.generate_king_moves(&mut mc);
        assert_eq!(mc.capture_count + mc.quiet_count, 0);
        mc.clear();

        // Expected: f1 quiet move + g1 castle = 2 moves
        let g = Position::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1");
        g.generate_king_moves(&mut mc);
        assert_eq!(mc.capture_count + mc.quiet_count, 2);
        mc.clear();

        // Expected: f1 quiet move = 1 move
        let g = Position::new_from_fen("rnbqkbn1/pppppprp/8/8/8/8/PPPPPP1P/RNBQK2R w KQq - 0 1");
        g.generate_king_moves(&mut mc);
        assert_eq!(mc.capture_count + mc.quiet_count, 1);
        mc.clear();

        // Expected: f1 quiet = 1 moves
        let g = Position::new_from_fen("rnb1kb1r/ppppppPp/8/q7/8/8/PPP1PPPP/RNBQK2R w KQkq - 0 1");
        g.generate_king_moves(&mut mc);
        assert_eq!(mc.capture_count + mc.quiet_count, 2);
        mc.clear();
    }
}
