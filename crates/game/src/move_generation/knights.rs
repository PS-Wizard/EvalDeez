use crate::position::Position;
use raw::KNIGHT_ATTACKS;
use types::move_collector::MoveCollector;
use types::move_type::{Move, MoveType::*};
use types::piece_type::Piece;

impl Position {
    #[inline(always)]
    /// Generates all pseudo-legal knight moves (doesn't check for pins or checks)
    /// Legality will be verified with make/unmake
    pub fn generate_knight_moves(&self, collector: &mut MoveCollector) {
        let mut our_knights = self.friendly(Piece::Knight).0;
        let friendly = self.us().0;
        let enemy = self.them().0;

        while our_knights != 0 {
            let from = our_knights.trailing_zeros() as usize;
            our_knights &= our_knights - 1; // Pop LSB

            let attacks = KNIGHT_ATTACKS[from] & !friendly;

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
mod knight_moves {
    use crate::position::Position;
    use types::move_collector::MoveCollector;

    #[test]
    fn generate_knight_pseudo_legal() {
        // Initial game position should return 4 moves (2 knights, 2 moves each)
        let g = Position::new();
        let mut mc = MoveCollector::new();
        g.generate_knight_moves(&mut mc);
        assert_eq!(mc.capture_count + mc.quiet_count, 4);
        mc.clear();

        // Expected 7 quiet moves, 3 captures = total 10
        let g = Position::new_from_fen(
            "rnbqk1nr/ppp2pp1/8/2p1p1p1/8/2N2N2/PP1PPPPP/R1BQKBbb w Qkq - 0 1",
        );
        g.generate_knight_moves(&mut mc);
        assert_eq!(mc.capture_count + mc.quiet_count, 10);
        assert_eq!(mc.capture_count, 3);
        assert_eq!(mc.quiet_count, 7);
        mc.clear();

        // Will generate more moves than legal (includes pinned knight moves)
        let g =
            Position::new_from_fen("rnb1k1n1/ppppqppp/8/5N2/7b/3N2N1/PPPP2PP/r1N1KB1R w Kq - 0 1");
        g.generate_knight_moves(&mut mc);
        println!(
            "Generated {} pseudo-legal knight moves (includes pinned)",
            mc.capture_count + mc.quiet_count
        );

        assert_eq!(mc.capture_count + mc.quiet_count, 17);
        assert_eq!(mc.capture_count, 3);
        assert_eq!(mc.quiet_count, 14);

        // Will be more than 3 since we're not filtering pinned knights
        mc.clear();
    }
}
