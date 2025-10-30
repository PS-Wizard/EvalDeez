use crate::position::Position;
use raw::{ROOK_ATTACKS, ROOK_MASKS};
use std::arch::x86_64::_pext_u64;
use types::move_collector::MoveCollector;
use types::move_type::{Move, MoveType::*};
use types::piece_type::Piece;

impl Position {
    #[inline(always)]
    /// Generates all pseudo-legal rook moves (doesn't check for pins or checks)
    /// Legality will be verified with make/unmake
    pub fn generate_rook_moves(&self, collector: &mut MoveCollector) {
        let mut our_rooks = self.friendly(Piece::Rook).0;
        let blockers = self.colors[0].0 | self.colors[1].0;
        let friendly = self.us().0;
        let enemy = self.them().0;

        while our_rooks != 0 {
            let from = our_rooks.trailing_zeros() as usize;
            our_rooks &= our_rooks - 1; // Pop LSB

            let mask_idx = unsafe { _pext_u64(blockers, ROOK_MASKS[from]) as usize };
            let attacks = ROOK_ATTACKS[from][mask_idx] & !friendly;

            // Generate capture moves
            let mut capture_bb = attacks & enemy;
            while capture_bb != 0 {
                let to = capture_bb.trailing_zeros() as usize;
                capture_bb &= capture_bb - 1;
                collector.push_capture(Move::new(from, to, Capture));
            }

            // Generate quiet moves
            let mut quiet_bb = attacks & !enemy;
            while quiet_bb != 0 {
                let to = quiet_bb.trailing_zeros() as usize;
                quiet_bb &= quiet_bb - 1;
                collector.push_quiet(Move::new(from, to, Quiet));
            }
        }
    }
}

#[cfg(test)]
mod rook_moves {
    use crate::position::Position;
    use types::move_collector::MoveCollector;

    #[test]
    fn generate_rook_pseudo_legal() {
        // Initial game position should return 0 moves (rooks blocked)
        let g = Position::new();
        let mut mc = MoveCollector::new();
        g.generate_rook_moves(&mut mc);
        assert_eq!(mc.capture_count + mc.quiet_count, 0);
        mc.clear();

        // Expected 13 quiet moves, 2 captures = 15 total
        let g = Position::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/8/R3KBNR w KQkq - 0 1");
        g.generate_rook_moves(&mut mc);
        assert_eq!(mc.capture_count + mc.quiet_count, 15);
        assert_eq!(mc.capture_count, 2);
        assert_eq!(mc.quiet_count, 13);
        mc.clear();

        // Will generate moves including pinned rooks
        let g = Position::new_from_fen("rn2kbnr/pppppppp/8/8/8/7P/P1PPPPP1/Kb1R3q w kq - 0 1");
        g.generate_rook_moves(&mut mc);
        assert_eq!(mc.quiet_count, 4);
        assert_eq!(mc.capture_count, 2);
        mc.clear();
    }
}
