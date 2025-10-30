use crate::position::Position;
use raw::PAWN_ATTACKS;
use types::move_collector::MoveCollector;
use types::move_type::{Move, MoveType::*};
use types::piece_type::{Color::*, Piece::*};

impl Position {
    /// Generates all pseudo-legal pawn moves (doesn't check for pins or checks)
    /// Legality will be verified with make/unmake
    #[inline(always)]
    pub fn generate_pawn_moves(&self, collector: &mut MoveCollector) {
        if self.side_to_move == White {
            self.generate_white_pawn_moves(collector);
        } else {
            self.generate_black_pawn_moves(collector);
        }
    }

    /// Generates all pseudo-legal white pawn moves
    fn generate_white_pawn_moves(&self, collector: &mut MoveCollector) {
        let pawns = self.friendly(Pawn).0;
        let empty = !(self.colors[0].0 | self.colors[1].0);
        let enemies = self.them().0;

        // Process each pawn
        let mut pawn_bb = pawns;
        while pawn_bb != 0 {
            let from = pawn_bb.trailing_zeros() as usize;
            pawn_bb &= pawn_bb - 1;

            // Single push
            let push_to = from + 8;
            if push_to < 64 && (empty >> push_to) & 1 != 0 {
                if push_to >= 56 {
                    // Promotion
                    collector.push_quiet(Move::new(from, push_to, PromotionQueen));
                    collector.push_quiet(Move::new(from, push_to, PromotionRook));
                    collector.push_quiet(Move::new(from, push_to, PromotionBishop));
                    collector.push_quiet(Move::new(from, push_to, PromotionKnight));
                } else {
                    collector.push_quiet(Move::new(from, push_to, Quiet));

                    // Double push (only if single push was successful and pawn on rank 2)
                    if from >= 8 && from < 16 {
                        let double_to = from + 16;
                        if (empty >> double_to) & 1 != 0 {
                            collector.push_quiet(Move::new(from, double_to, DoublePush));
                        }
                    }
                }
            }

            // Captures
            let attacks = PAWN_ATTACKS[0][from] & enemies;
            let mut attack_bb = attacks;
            while attack_bb != 0 {
                let to = attack_bb.trailing_zeros() as usize;
                attack_bb &= attack_bb - 1;

                if to >= 56 {
                    // Capture promotion
                    collector.push_capture(Move::new(from, to, CapturePromotionQueen));
                    collector.push_capture(Move::new(from, to, CapturePromotionRook));
                    collector.push_capture(Move::new(from, to, CapturePromotionBishop));
                    collector.push_capture(Move::new(from, to, CapturePromotionKnight));
                } else {
                    collector.push_capture(Move::new(from, to, Capture));
                }
            }
        }

        // En passant
        if let Some(ep_sq) = self.en_passant {
            self.generate_white_en_passant(collector, ep_sq as usize);
        }
    }

    /// Generates pseudo-legal white en passant moves
    fn generate_white_en_passant(&self, collector: &mut MoveCollector, ep_sq: usize) {
        let pawns = self.friendly(Pawn).0;
        let ep_target = 1u64 << ep_sq;

        let mut pawn_bb = pawns;
        while pawn_bb != 0 {
            let from = pawn_bb.trailing_zeros() as usize;
            pawn_bb &= pawn_bb - 1;

            // Can this pawn capture en passant?
            if (PAWN_ATTACKS[0][from] & ep_target) != 0 {
                collector.push_capture(Move::new(from, ep_sq, EnPassant));
            }
        }
    }

    /// Generates all pseudo-legal black pawn moves
    fn generate_black_pawn_moves(&self, collector: &mut MoveCollector) {
        let pawns = self.friendly(Pawn).0;
        let empty = !(self.colors[0].0 | self.colors[1].0);
        let enemies = self.them().0;

        // Process each pawn
        let mut pawn_bb = pawns;
        while pawn_bb != 0 {
            let from = pawn_bb.trailing_zeros() as usize;
            pawn_bb &= pawn_bb - 1;

            // Single push (black pawns move down: from - 8)
            if from >= 8 {
                let push_to = from - 8;
                if (empty >> push_to) & 1 != 0 {
                    if push_to < 8 {
                        // Promotion (rank 1)
                        collector.push_quiet(Move::new(from, push_to, PromotionQueen));
                        collector.push_quiet(Move::new(from, push_to, PromotionRook));
                        collector.push_quiet(Move::new(from, push_to, PromotionBishop));
                        collector.push_quiet(Move::new(from, push_to, PromotionKnight));
                    } else {
                        collector.push_quiet(Move::new(from, push_to, Quiet));

                        // Double push (only if single push was successful and pawn on rank 7)
                        if from >= 48 && from < 56 {
                            let double_to = from - 16;
                            if (empty >> double_to) & 1 != 0 {
                                collector.push_quiet(Move::new(from, double_to, DoublePush));
                            }
                        }
                    }
                }
            }

            // Captures
            let attacks = PAWN_ATTACKS[1][from] & enemies;
            let mut attack_bb = attacks;
            while attack_bb != 0 {
                let to = attack_bb.trailing_zeros() as usize;
                attack_bb &= attack_bb - 1;

                if to < 8 {
                    // Capture promotion (rank 1)
                    collector.push_capture(Move::new(from, to, CapturePromotionQueen));
                    collector.push_capture(Move::new(from, to, CapturePromotionRook));
                    collector.push_capture(Move::new(from, to, CapturePromotionBishop));
                    collector.push_capture(Move::new(from, to, CapturePromotionKnight));
                } else {
                    collector.push_capture(Move::new(from, to, Capture));
                }
            }
        }

        // En passant
        if let Some(ep_sq) = self.en_passant {
            self.generate_black_en_passant(collector, ep_sq as usize);
        }
    }

    /// Generates pseudo-legal black en passant moves
    fn generate_black_en_passant(&self, collector: &mut MoveCollector, ep_sq: usize) {
        let pawns = self.friendly(Pawn).0;
        let ep_target = 1u64 << ep_sq;

        let mut pawn_bb = pawns;
        while pawn_bb != 0 {
            let from = pawn_bb.trailing_zeros() as usize;
            pawn_bb &= pawn_bb - 1;

            // Can this pawn capture en passant?
            if (PAWN_ATTACKS[1][from] & ep_target) != 0 {
                collector.push_capture(Move::new(from, ep_sq, EnPassant));
            }
        }
    }
}

#[cfg(test)]
mod pawns {
    use crate::position::Position;
    use types::move_collector::MoveCollector;

    #[test]
    fn test_pawn_pseudo_legal() {
        // Initial position should be 16 moves
        let g = Position::new();
        let mut mc = MoveCollector::new();
        g.generate_pawn_moves(&mut mc);
        assert_eq!(mc.capture_count + mc.quiet_count, 16);
        mc.clear();

        // En passant on b6 - will generate pseudo-legal moves
        let g =
            Position::new_from_fen("rn2k1nr/p1ppp1pp/8/1pP5/8/7P/PP2P1P1/RNBQK2R w KQkq b6 0 1");
        g.generate_pawn_moves(&mut mc);
        // Will generate more moves than legal (includes pinned moves)
        println!(
            "Generated {} pseudo-legal pawn moves",
            mc.capture_count + mc.quiet_count
        );
        assert_eq!(mc.quiet_count, 10);
        assert_eq!(mc.capture_count, 1);

        mc.clear();

        // Pinned pawn - will generate moves but make/unmake will reject illegal ones
        let g =
            Position::new_from_fen("1nb1k1nr/pppppppp/4r3/b7/7q/4P3/2PB1PPP/RN1QKB1R w KQk - 0 1");
        g.generate_pawn_moves(&mut mc);
        println!(
            "Generated {} pseudo-legal pawn moves (includes pinned)",
            mc.capture_count + mc.quiet_count
        );
        assert_eq!(8, mc.quiet_count);
        mc.clear();

        // Capture to promotion -> Expected: 4 promotion moves
        let g = Position::new_from_fen("rn2k3/pPp1ppPp/8/b2p3q/8/8/3B4/RNKQ1B1R w KQkq - 0 1");
        g.generate_pawn_moves(&mut mc);
        // Atleast 4 capture -> promo moves 
        assert_eq!(mc.capture_count, 4);

        // Atleast 4 quiet -> promo moves 
        assert_eq!(mc.quiet_count, 4);
        mc.clear();
    }
}
