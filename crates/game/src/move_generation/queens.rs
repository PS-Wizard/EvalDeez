use crate::position::Position;
use raw::{BISHOP_ATTACKS, BISHOP_MASKS, ROOK_ATTACKS, ROOK_MASKS};
use std::arch::x86_64::_pext_u64;
use types::move_collector::MoveCollector;
use types::move_type::{Move, MoveType::*};
use types::piece_type::Piece;

impl Position {
    #[inline(always)]
    /// Generates all pseudo-legal queen moves (doesn't check for pins or checks)
    /// Legality will be verified with make/unmake
    /// Uses PEXT attack tables for both rook and bishop move patterns
    pub fn generate_queen_moves(&self, collector: &mut MoveCollector) {
        let mut our_queens = self.friendly(Piece::Queen).0;
        let blockers = self.colors[0].0 | self.colors[1].0;
        let friendly = self.us().0;
        let enemy = self.them().0;

        while our_queens != 0 {
            let from = our_queens.trailing_zeros() as usize;
            our_queens &= our_queens - 1; // Pop LSB

            // Queen moves = Rook moves + Bishop moves
            let bishop_idx = unsafe { _pext_u64(blockers, BISHOP_MASKS[from]) as usize };
            let rook_idx = unsafe { _pext_u64(blockers, ROOK_MASKS[from]) as usize };
            let attacks =
                (BISHOP_ATTACKS[from][bishop_idx] | ROOK_ATTACKS[from][rook_idx]) & !friendly;

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
mod queen_moves {
    use crate::position::Position;
    use types::move_collector::MoveCollector;

    #[test]
    fn generate_queen_pseudo_legal() {
        // Initial game position should return 0 moves (queen blocked)
        let g = Position::new();
        let mut mc = MoveCollector::new();
        g.generate_queen_moves(&mut mc);
        assert_eq!(mc.capture_count + mc.quiet_count, 0);
        mc.clear();

        // Expected 9 quiet moves, 1 capture = total 10
        let g = Position::new_from_fen(
            "rnbqk1nr/ppp2pp1/8/2p1p1p1/8/2N2N2/PP2PPPP/R1BQKB2 w Qkq - 0 1",
        );
        g.generate_queen_moves(&mut mc);
        assert_eq!(mc.capture_count, 1);
        assert_eq!(mc.quiet_count, 9);
        mc.clear();

        // Will generate moves including pinned queen moves
        let g = Position::new_from_fen("rn2kbnr/pppppppp/8/8/b7/7P/PP1PPPP1/q2QKBNR w Kkq - 0 1");
        g.generate_queen_moves(&mut mc);
        assert_eq!(mc.quiet_count, 4);
        assert_eq!(mc.capture_count, 2);
    }
}
