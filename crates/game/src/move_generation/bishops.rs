use crate::position::Position;
use raw::{BISHOP_ATTACKS, BISHOP_MASKS};
use std::arch::x86_64::_pext_u64;
use types::move_collector::MoveCollector;
use types::move_type::{Move, MoveType::*};
use types::piece_type::Piece;

impl Position {
    #[inline(always)]
    /// Generates all pseudo-legal bishop moves (doesn't check for pins or checks)
    /// Legality will be verified with make/unmake
    pub fn generate_bishop_moves(&self, collector: &mut MoveCollector) {
        let mut our_bishops = self.friendly(Piece::Bishop).0;
        let blockers = self.colors[0].0 | self.colors[1].0;
        let friendly = self.us().0;
        let enemy = self.them().0;

        while our_bishops != 0 {
            let from = our_bishops.trailing_zeros() as usize;
            our_bishops &= our_bishops - 1; // Pop LSB

            let mask_idx = unsafe { _pext_u64(blockers, BISHOP_MASKS[from]) as usize };
            let attacks = BISHOP_ATTACKS[from][mask_idx] & !friendly;

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
mod bishop_moves {
    use crate::position::Position;
    use types::move_collector::MoveCollector;

    #[test]
    fn generate_bishop_pseudo_legal() {
        // Initial game position should return 0 moves (bishops blocked)
        let g = Position::new();
        let mut mc = MoveCollector::new();
        g.generate_bishop_moves(&mut mc);
        assert_eq!(mc.capture_count + mc.quiet_count, 0);
        mc.clear();

        // Expected 14 quiet moves, 3 captures = total 17
        let g =
            Position::new_from_fen("rnbqkbnr/pppppppp/8/3B4/8/4B3/PPP2PPP/RN1QK1NR w KQkq - 0 1");
        g.generate_bishop_moves(&mut mc);
        assert_eq!(mc.capture_count + mc.quiet_count, 17);
        assert_eq!(mc.capture_count, 3);
        assert_eq!(mc.quiet_count, 14);
        mc.clear();
    }
}
