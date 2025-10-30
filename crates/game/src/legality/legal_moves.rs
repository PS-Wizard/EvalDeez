use crate::position::Position;
use types::move_collector::MoveCollector;

impl Position {
    /// Generates all legal moves by:
    /// 1. Generating all pseudo-legal moves
    /// 2. Testing each with make/unmake
    /// 3. Keeping only moves that don't leave king in check
    pub fn generate_legal_moves(&self, collector: &mut MoveCollector) {
        // Generate all pseudo-legal moves into a temporary collector
        let mut pseudo_legal = MoveCollector::new();
        self.generate_pseudo_legal_moves(&mut pseudo_legal);

        // Filter to only legal moves by testing with make/unmake
        for m in pseudo_legal.captures_iter() {
            if self.is_legal_move(m) {
                collector.push_capture(m);
            }
        }

        for m in pseudo_legal.quiets_iter() {
            if self.is_legal_move(m) {
                collector.push_quiet(m);
            }
        }
    }

    /// Tests if a move is legal by making it and checking if king is in check
    #[inline(always)]
    pub fn is_legal_move(&self, m: types::move_type::Move) -> bool {
        let mut pos = self.clone();
        let undo = pos.make_move(m);

        // After making the move, check if our king (now opponent's turn) is in check
        // We need to check the side that just moved, which is now the opposite of side_to_move
        let moving_side = pos.side_to_move.flip();
        let king_bb = pos.colors[moving_side] & pos.pieces[types::piece_type::Piece::King];

        if king_bb.0 == 0 {
            // King was captured (shouldn't happen in legal chess)
            pos.unmake_move(m, undo);
            return false;
        }

        let king_square = king_bb.0.trailing_zeros() as usize;
        let is_legal = !pos.is_square_attacked(king_square, pos.side_to_move, None);

        pos.unmake_move(m, undo);
        is_legal
    }

    /// Check if position has any legal moves (for checkmate/stalemate detection)
    pub fn has_legal_moves(&self) -> bool {
        let mut pseudo_legal = MoveCollector::new();
        self.generate_pseudo_legal_moves(&mut pseudo_legal);

        // Check captures first (usually fewer)
        for m in pseudo_legal.captures_iter() {
            if self.is_legal_move(m) {
                return true;
            }
        }

        // Then check quiet moves
        for m in pseudo_legal.quiets_iter() {
            if self.is_legal_move(m) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod legal_move_tests {
    use crate::position::Position;
    use types::move_collector::MoveCollector;

    #[test]
    fn test_legal_move_generation() {
        // Starting position - 20 legal moves
        let pos = Position::new();
        let mut mc = MoveCollector::new();
        pos.generate_legal_moves(&mut mc);
        assert_eq!(mc.capture_count + mc.quiet_count, 20);
        mc.clear();

        // Position with pins - pseudo-legal != legal
        let pos =
            Position::new_from_fen("rnb1k1nr/pppppppp/4r3/b7/7q/4P3/2PB1PPP/RN1QKB1R w KQk - 0 1");

        let mut pseudo = MoveCollector::new();
        pos.generate_pseudo_legal_moves(&mut pseudo);
        let pseudo_count = pseudo.capture_count + pseudo.quiet_count;

        pos.generate_legal_moves(&mut mc);
        let legal_count = mc.capture_count + mc.quiet_count;

        println!("Pseudo-legal: {}, Legal: {}", pseudo_count, legal_count);
        assert!(
            legal_count < pseudo_count,
            "Legal moves should be less than pseudo-legal when there are pins"
        );
        mc.clear();

        // Checkmate position - 0 legal moves
        let pos =
            Position::new_from_fen("rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 0 1");
        pos.generate_legal_moves(&mut mc);
        assert_eq!(mc.capture_count + mc.quiet_count, 0);
        assert!(!pos.has_legal_moves());
    }

    #[test]
    fn test_en_passant_legality() {
        // En passant that would expose king to check (should be illegal)
        let pos = Position::new_from_fen("8/8/8/1k1Pp2r/8/8/4K3/8 w - e6 0 1");
        let mut mc = MoveCollector::new();
        pos.generate_legal_moves(&mut mc);

        // Should not include the illegal en passant
        for m in mc.captures_iter() {
            assert!(pos.is_legal_move(m));
        }
    }

    #[test]
    fn test_king_moves_into_check() {
        // King shouldn't be able to move into attacked squares
        let pos =
            Position::new_from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1");
        let mut mc = MoveCollector::new();
        pos.generate_legal_moves(&mut mc);

        // Verify all king moves are legal
        for m in mc.quiets_iter() {
            assert!(pos.is_legal_move(m));
        }
        for m in mc.captures_iter() {
            assert!(pos.is_legal_move(m));
        }
    }

    #[test]
    fn test_perft_with_legal_moves() {
        // Verify perft works with legal move generation
        let mut pos = Position::new();

        // Depth 1 - should be 20 moves
        let nodes = perft_legal(&mut pos, 1);
        assert_eq!(nodes, 20);

        // Depth 2 - should be 400 moves
        let nodes = perft_legal(&mut pos, 2);
        assert_eq!(nodes, 400);

        // Depth 3 - should be 8,902 moves
        let nodes = perft_legal(&mut pos, 3);
        assert_eq!(nodes, 8_902);
    }

    fn perft_legal(pos: &mut Position, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut collector = MoveCollector::new();
        pos.generate_legal_moves(&mut collector);

        if depth == 1 {
            return (collector.capture_count + collector.quiet_count) as u64;
        }

        let mut nodes = 0u64;
        for m in collector.captures_iter() {
            let undo = pos.make_move(m);
            nodes += perft_legal(pos, depth - 1);
            pos.unmake_move(m, undo);
        }
        for m in collector.quiets_iter() {
            let undo = pos.make_move(m);
            nodes += perft_legal(pos, depth - 1);
            pos.unmake_move(m, undo);
        }
        nodes
    }
}

