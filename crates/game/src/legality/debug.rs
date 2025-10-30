use crate::position::Position;
use std::collections::HashMap;
use types::move_collector::MoveCollector;

impl Position {
    /// Extended perft divide that shows ALL moves with counts
    pub fn perft_divide_detailed(&mut self, depth: u8) {
        let mut collector = MoveCollector::new();
        self.generate_legal_moves(&mut collector);

        let mut move_counts: Vec<(String, u64)> = Vec::new();
        let mut total = 0u64;

        println!("\n=== CAPTURES ===");
        for m in collector.captures_iter() {
            let undo = self.make_move(m);
            let count = if depth <= 1 { 1 } else { self.perft(depth - 1) };
            self.unmake_move(m, undo);

            let move_str = format!("{}", m);
            println!("{}: {}", move_str, count);
            move_counts.push((move_str, count));
            total += count;
        }

        println!("\n=== QUIET MOVES ===");
        for m in collector.quiets_iter() {
            let undo = self.make_move(m);
            let count = if depth <= 1 { 1 } else { self.perft(depth - 1) };
            self.unmake_move(m, undo);

            let move_str = format!("{}", m);
            println!("{}: {}", move_str, count);
            move_counts.push((move_str, count));
            total += count;
        }

        println!("\n=== SUMMARY ===");
        println!(
            "Total moves: {}",
            collector.capture_count + collector.quiet_count
        );
        println!("Total nodes: {}", total);
        println!("Captures: {}", collector.capture_count);
        println!("Quiet: {}", collector.quiet_count);
    }

    /// Compare with expected perft results
    pub fn perft_compare(&mut self, depth: u8, expected: &HashMap<String, u64>) {
        let mut collector = MoveCollector::new();
        self.generate_legal_moves(&mut collector);

        let mut found_moves: HashMap<String, u64> = HashMap::new();

        for m in collector.captures_iter() {
            let undo = self.make_move(m);
            let count = if depth <= 1 { 1 } else { self.perft(depth - 1) };
            self.unmake_move(m, undo);
            found_moves.insert(format!("{}", m), count);
        }

        for m in collector.quiets_iter() {
            let undo = self.make_move(m);
            let count = if depth <= 1 { 1 } else { self.perft(depth - 1) };
            self.unmake_move(m, undo);
            found_moves.insert(format!("{}", m), count);
        }

        // Find missing moves
        println!("\n=== MISSING MOVES ===");
        let mut missing_count = 0u64;
        for (mv, count) in expected.iter() {
            if !found_moves.contains_key(mv) {
                println!("MISSING: {} (should have {} nodes)", mv, count);
                missing_count += count;
            }
        }

        // Find extra moves
        println!("\n=== EXTRA MOVES ===");
        let mut extra_count = 0u64;
        for (mv, count) in found_moves.iter() {
            if !expected.contains_key(mv) {
                println!("EXTRA: {} ({} nodes)", mv, count);
                extra_count += count;
            }
        }

        // Find count mismatches
        println!("\n=== COUNT MISMATCHES ===");
        for (mv, count) in found_moves.iter() {
            if let Some(&expected_count) = expected.get(mv) {
                if *count != expected_count {
                    println!("{}: got {} but expected {}", mv, count, expected_count);
                }
            }
        }

        let total_found: u64 = found_moves.values().sum();
        let total_expected: u64 = expected.values().sum();

        println!("\n=== TOTALS ===");
        println!("Found: {} nodes", total_found);
        println!("Expected: {} nodes", total_expected);
        println!(
            "Difference: {} nodes",
            (total_found as i64 - total_expected as i64).abs()
        );
        println!("Missing nodes: {}", missing_count);
        println!("Extra nodes: {}", extra_count);
    }

    /// Debug a specific move path
    pub fn debug_move_path(&mut self, moves: &[&str]) {
        println!("\n=== DEBUGGING MOVE PATH ===");
        for (i, move_str) in moves.iter().enumerate() {
            println!("\nDepth {}: Making move {}", i, move_str);

            let mut collector = MoveCollector::new();
            self.generate_legal_moves(&mut collector);

            // Find the move
            let mut found = false;
            for m in collector.captures_iter().chain(collector.quiets_iter()) {
                if format!("{}", m) == *move_str {
                    println!("  Found move: {:?}", m);
                    let _ = self.make_move(m);
                    println!("  Position after move:");
                    println!("  Side to move: {:?}", self.side_to_move);
                    println!("  En passant: {:?}", self.en_passant);
                    println!("  Castling: {:?}", self.castling_rights);
                    found = true;

                    // Don't unmake - we want to continue down the path
                    break;
                }
            }

            if !found {
                println!("  ERROR: Move {} not found!", move_str);
                println!("  Available moves:");
                for m in collector.captures_iter().chain(collector.quiets_iter()) {
                    println!("    {}", m);
                }
                break;
            }
        }
    }
}

#[cfg(test)]
mod perft_debug_tests {
    use super::*;

    #[test]
    #[ignore]
    fn debug_kiwipete() {
        let mut pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        )
        .unwrap();

        println!("=== KIWIPETE POSITION ===");
        pos.perft_divide_detailed(4);
    }

    #[test]
    fn debug_kiwipete_depth1() {
        let pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        )
        .unwrap();

        let mut mc = MoveCollector::new();
        pos.generate_legal_moves(&mut mc);
        println!("Total depth 1 moves: {}", mc.capture_count + mc.quiet_count);
        println!("Expected: 48");

        // List all moves
        for m in mc.captures_iter() {
            println!("CAPTURE: {}", m);
        }
        for m in mc.quiets_iter() {
            println!("QUIET: {}", m);
        }
    }
}

#[cfg(test)]
mod kiwipete_debug {
    use crate::position::Position;
    use types::move_collector::MoveCollector;

    #[test]
    fn check_move_counts() {
        let pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        )
        .unwrap();

        println!("\n=== KIWIPETE MOVE COUNT CHECK ===");

        // Check depth 1 first
        let mut mc = MoveCollector::new();
        pos.generate_legal_moves(&mut mc);
        let depth1_moves = mc.capture_count + mc.quiet_count;
        println!("Depth 1 moves: {} (expected: 48)", depth1_moves);
        assert_eq!(depth1_moves, 48, "Depth 1 should have 48 moves");

        // List all depth 1 moves
        println!("\nAll depth 1 moves:");
        for m in mc.captures_iter() {
            println!("  CAPTURE: {}", m);
        }
        for m in mc.quiets_iter() {
            println!("  QUIET: {}", m);
        }
    }

    #[test]
    fn check_castling_generation() {
        let pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        )
        .unwrap();

        println!("\n=== CHECKING CASTLING ===");
        println!("Castling rights: {:?}", pos.castling_rights);
        println!(
            "King square: {}",
            pos.pieces[types::piece_type::Piece::King]
                .0
                .trailing_zeros()
        );

        let mut mc = MoveCollector::new();
        pos.generate_legal_moves(&mut mc);

        let mut castle_count = 0;
        for m in mc.quiets_iter() {
            if m.move_type() == types::move_type::MoveType::Castle {
                println!("  Castle move: {}", m);
                castle_count += 1;
            }
        }
        println!("Total castle moves: {} (expected: 2)", castle_count);
        assert_eq!(castle_count, 2, "Should have 2 castling moves");
    }

    #[test]
    fn check_pseudo_vs_legal() {
        let pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        )
        .unwrap();

        println!("\n=== PSEUDO-LEGAL VS LEGAL ===");

        let mut pseudo = MoveCollector::new();
        pos.generate_pseudo_legal_moves(&mut pseudo);
        let pseudo_count = pseudo.capture_count + pseudo.quiet_count;

        let mut legal = MoveCollector::new();
        pos.generate_legal_moves(&mut legal);
        let legal_count = legal.capture_count + legal.quiet_count;

        println!("Pseudo-legal moves: {}", pseudo_count);
        println!("Legal moves: {}", legal_count);
        println!("Filtered out: {}", pseudo_count - legal_count);

        // Check which moves were filtered
        let mut pseudo_moves = std::collections::HashSet::new();
        for m in pseudo.captures_iter().chain(pseudo.quiets_iter()) {
            pseudo_moves.insert(format!("{}", m));
        }

        let mut legal_moves = std::collections::HashSet::new();
        for m in legal.captures_iter().chain(legal.quiets_iter()) {
            legal_moves.insert(format!("{}", m));
        }

        println!("\nFiltered out (illegal) moves:");
        for mv in pseudo_moves.difference(&legal_moves) {
            println!("  {}", mv);
        }
    }

    #[test]
    fn depth_by_depth_check() {
        let mut pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        )
        .unwrap();

        println!("\n=== DEPTH BY DEPTH CHECK ===");

        let depths_expected = [(1, 48), (2, 2039), (3, 97862), (4, 4085603)];

        for (depth, expected) in depths_expected {
            let nodes = pos.perft(depth);
            let status = if nodes == expected { "✓" } else { "✗" };
            println!(
                "Depth {}: {} nodes {} (expected: {})",
                depth, nodes, status, expected
            );

            if nodes != expected {
                let diff = (nodes as i64 - expected as i64).abs();
                println!("  Difference: {} nodes", diff);
                break;
            }
        }
    }

    #[test]
    #[ignore]
    fn find_problematic_move() {
        let mut pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        )
        .unwrap();

        println!("\n=== FINDING PROBLEMATIC MOVE ===");

        // Reference counts from Stockfish for depth 3
        let reference = vec![
            ("a2a3", 2134),
            ("a2a4", 2046),
            ("b2b3", 2048),
            ("g2g3", 1947),
            ("g2g4", 1906),
            ("g2h3", 2149),
            // Add more as needed
        ];

        let mut collector = MoveCollector::new();
        pos.generate_legal_moves(&mut collector);

        for m in collector.captures_iter().chain(collector.quiets_iter()) {
            let move_str = format!("{}", m);
            let undo = pos.make_move(m);
            let count = pos.perft(3);
            pos.unmake_move(m, undo);

            // Check if this matches reference
            if let Some((_, expected)) = reference.iter().find(|(mv, _)| *mv == move_str) {
                if count != *expected {
                    println!(
                        "MISMATCH: {} - got {}, expected {}",
                        move_str, count, expected
                    );
                } else {
                    println!("OK: {} - {}", move_str, count);
                }
            } else {
                println!("NOT IN REFERENCE: {} - {}", move_str, count);
            }
        }
    }
}

#[cfg(test)]
mod legality_debug {
    use crate::position::Position;
    use types::move_collector::MoveCollector;

    #[test]
    fn test_is_legal_move_logic() {
        // Simple position where we can verify legality
        let pos =
            Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        let mut mc = MoveCollector::new();
        pos.generate_pseudo_legal_moves(&mut mc);

        println!("\n=== Testing is_legal_move for starting position ===");
        println!("All moves should be legal in starting position");

        let mut illegal_count = 0;
        for m in mc.captures_iter().chain(mc.quiets_iter()) {
            if !pos.is_legal_move(m) {
                println!("ERROR: Move {} marked as illegal!", m);
                illegal_count += 1;
            }
        }

        assert_eq!(
            illegal_count, 0,
            "No moves should be illegal in starting position"
        );
        println!(
            "✓ All {} moves are legal",
            mc.capture_count + mc.quiet_count
        );
    }

    #[test]
    fn test_pinned_piece_detection() {
        // Position with a pinned bishop
        let pos = Position::from_fen("rnb1kbn1/pppppppp/8/8/8/4B2P/PP1PPPP1/RN1QKB1r w Qq - 0 1")
            .unwrap();

        println!("\n=== Testing pinned bishop detection ===");

        let mut pseudo = MoveCollector::new();
        pos.generate_pseudo_legal_moves(&mut pseudo);
        let pseudo_count = pseudo.capture_count + pseudo.quiet_count;

        let mut legal = MoveCollector::new();
        pos.generate_legal_moves(&mut legal);
        let legal_count = legal.capture_count + legal.quiet_count;

        println!("Pseudo-legal: {}", pseudo_count);
        println!("Legal: {}", legal_count);
        println!("Filtered: {}", pseudo_count - legal_count);

        // The bishop on e3 is pinned, so its moves should be filtered
        assert!(
            legal_count < pseudo_count,
            "Pinned piece moves should be filtered"
        );
    }

    #[test]
    fn test_move_leaves_king_in_check() {
        // Position where moving a piece would leave king in check
        let pos = Position::from_fen("4k3/8/8/8/8/4n3/4P3/4K3 w - - 0 1").unwrap();

        println!("\n=== Testing move that leaves king in check ===");

        // The pawn on e2 is pinned by the knight on e3
        // Moving it should be illegal
        let mut pseudo = MoveCollector::new();
        pos.generate_pseudo_legal_moves(&mut pseudo);

        let mut legal = MoveCollector::new();
        pos.generate_legal_moves(&mut legal);

        println!(
            "Pseudo-legal moves: {}",
            pseudo.capture_count + pseudo.quiet_count
        );
        println!("Legal moves: {}", legal.capture_count + legal.quiet_count);

        // Check if e2e3 (capturing the knight) is legal
        for m in pseudo.captures_iter() {
            if m.from() == 12 && m.to() == 20 {
                // e2 to e3
                let is_legal = pos.is_legal_move(m);
                println!(
                    "e2e3 (capture knight): {}",
                    if is_legal { "LEGAL" } else { "ILLEGAL" }
                );
                assert!(is_legal, "Capturing the attacking piece should be legal");
            }
        }

        // Check if e2e4 is illegal (would leave king in check)
        for m in pseudo.quiets_iter() {
            if m.from() == 12 && m.to() == 28 {
                // e2 to e4
                let is_legal = pos.is_legal_move(m);
                println!(
                    "e2e4 (quiet): {}",
                    if is_legal { "LEGAL" } else { "ILLEGAL" }
                );
                assert!(!is_legal, "Moving pinned piece should be illegal");
            }
        }
    }

    #[test]
    fn debug_specific_kiwipete_move() {
        // Test the specific move a2a3 from Kiwipete
        let mut pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        )
        .unwrap();

        println!("\n=== Testing a2a3 from Kiwipete ===");

        // Make the move a2a3
        let mut mc = MoveCollector::new();
        pos.generate_legal_moves(&mut mc);

        for m in mc.quiets_iter() {
            if format!("{}", m) == "a2a3" {
                println!("Found move a2a3");
                let undo = pos.make_move(m);

                println!("After a2a3:");
                println!("  Side to move: {:?}", pos.side_to_move);

                // Now count legal moves from this position
                let mut mc2 = MoveCollector::new();
                pos.generate_legal_moves(&mut mc2);
                let count = mc2.capture_count + mc2.quiet_count;
                println!("  Legal moves: {}", count);

                // Do perft(2) which should give us 2134
                let nodes = pos.perft(2);
                println!("  Perft(2): {} (expected: 2134)", nodes);

                if nodes != 2134 {
                    println!("  ERROR: Expected 2134 but got {}", nodes);
                    println!("  Difference: {}", (nodes as i64 - 2134).abs());

                    // Check if moves are being generated correctly
                    println!("\n  All moves from this position:");
                    for m2 in mc2.captures_iter() {
                        println!("    CAPTURE: {}", m2);
                    }
                    for m2 in mc2.quiets_iter() {
                        println!("    QUIET: {}", m2);
                    }
                }

                pos.unmake_move(m, undo);
                break;
            }
        }
    }

    #[test]
    fn verify_is_square_attacked() {
        let pos =
            Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        println!("\n=== Testing is_square_attacked ===");

        // White king on e1 (square 4) should not be attacked
        let king_sq = 4;
        let attacked_by_black =
            pos.is_square_attacked(king_sq, types::piece_type::Color::Black, None);
        println!("White king at e1 attacked by Black? {}", attacked_by_black);
        assert!(
            !attacked_by_black,
            "King should not be attacked in starting position"
        );

        // Test a simple attacked square
        let pos2 = Position::from_fen("4k3/8/8/8/4r3/8/8/4K3 w - - 0 1").unwrap();
        let white_king = 4; // e1
        let attacked = pos2.is_square_attacked(white_king, types::piece_type::Color::Black, None);
        println!("White king attacked by black rook? {}", attacked);
        assert!(attacked, "King should be attacked by rook");
    }
}

#[cfg(test)]
mod castling_legality_debug {
    use crate::position::Position;
    use types::move_collector::MoveCollector;

    #[test]
    fn test_black_castling_after_a2a3() {
        // Position after a2a3 from Kiwipete
        let pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/P1N2Q1p/1P1BBPPP/R3K2R b KQkq - 0 2",
        )
        .unwrap();

        println!("\n=== Testing Black Castling after a2a3 ===");
        println!("Castling rights: {:?}", pos.castling_rights);

        // Check if squares are attacked
        println!("\nKingside castle (e8-g8):");
        println!(
            "  e8 attacked? {}",
            pos.is_square_attacked(60, types::piece_type::Color::White, None)
        );
        println!(
            "  f8 attacked? {}",
            pos.is_square_attacked(61, types::piece_type::Color::White, None)
        );
        println!(
            "  g8 attacked? {}",
            pos.is_square_attacked(62, types::piece_type::Color::White, None)
        );

        println!("\nQueenside castle (e8-c8):");
        println!(
            "  e8 attacked? {}",
            pos.is_square_attacked(60, types::piece_type::Color::White, None)
        );
        println!(
            "  d8 attacked? {}",
            pos.is_square_attacked(59, types::piece_type::Color::White, None)
        );
        println!(
            "  c8 attacked? {}",
            pos.is_square_attacked(58, types::piece_type::Color::White, None)
        );

        // Generate moves
        let mut mc = MoveCollector::new();
        pos.generate_pseudo_legal_moves(&mut mc);

        println!("\nPseudo-legal castling moves:");
        for m in mc.quiets_iter() {
            if m.move_type() == types::move_type::MoveType::Castle {
                println!("  {}: is_legal = {}", m, pos.is_legal_move(m));
            }
        }

        // Check legal moves
        let mut legal = MoveCollector::new();
        pos.generate_legal_moves(&mut legal);

        println!("\nLegal castling moves:");
        for m in legal.quiets_iter() {
            if m.move_type() == types::move_type::MoveType::Castle {
                println!("  {}", m);
            }
        }
    }

    #[test]
    fn test_castling_through_attacked_square() {
        // White king, rook on h1, black rook attacks f1
        let pos = Position::from_fen("4k3/8/8/8/8/8/8/4K2r w K - 0 1").unwrap();

        println!("\n=== Testing Castling Through Attack ===");
        println!("White king on e1, black rook on h1 attacks f1");

        println!(
            "e1 attacked? {}",
            pos.is_square_attacked(4, types::piece_type::Color::Black, None)
        );
        println!(
            "f1 attacked? {}",
            pos.is_square_attacked(5, types::piece_type::Color::Black, None)
        );
        println!(
            "g1 attacked? {}",
            pos.is_square_attacked(6, types::piece_type::Color::Black, None)
        );

        let mut mc = MoveCollector::new();
        pos.generate_legal_moves(&mut mc);

        let mut has_castle = false;
        for m in mc.quiets_iter() {
            if m.move_type() == types::move_type::MoveType::Castle {
                println!("ERROR: Castle move found: {}", m);
                has_castle = true;
            }
        }

        assert!(
            !has_castle,
            "Should not be able to castle through attacked square"
        );
    }

    #[test]
    fn test_castling_into_check() {
        // White king, rook on h1, black rook attacks g1
        let pos = Position::from_fen("4k3/8/8/8/8/8/6r1/4K2R w K - 0 1").unwrap();

        println!("\n=== Testing Castling Into Check ===");
        println!("White king on e1, black rook on g2 attacks g1");

        println!(
            "g1 attacked? {}",
            pos.is_square_attacked(6, types::piece_type::Color::Black, None)
        );

        let mut mc = MoveCollector::new();
        pos.generate_legal_moves(&mut mc);

        let mut has_castle = false;
        for m in mc.quiets_iter() {
            if m.move_type() == types::move_type::MoveType::Castle {
                println!("ERROR: Castle move found: {}", m);
                has_castle = true;
            }
        }

        assert!(!has_castle, "Should not be able to castle into check");
    }

    #[test]
    fn compare_stockfish_after_a2a3() {
        // After a2a3, according to Stockfish, Black should have these moves
        // Let's verify our move generation matches
        let pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/P1N2Q1p/1P1BBPPP/R3K2R b KQkq - 0 2",
        )
        .unwrap();

        println!("\n=== Comparing with Stockfish after a2a3 ===");

        let mut mc = MoveCollector::new();
        pos.generate_legal_moves(&mut mc);

        let move_count = mc.capture_count + mc.quiet_count;
        println!("Our move count: {}", move_count);
        println!("Expected: 44 (or close to it)");

        // Stockfish should say Black CAN castle kingside but NOT queenside
        // because c8 is attacked by the bishop on e2
        println!("\nChecking specific squares:");
        println!(
            "c8 (queenside castle destination) attacked by White? {}",
            pos.is_square_attacked(58, types::piece_type::Color::White, None)
        );
        println!(
            "d8 (queenside castle path) attacked by White? {}",
            pos.is_square_attacked(59, types::piece_type::Color::White, None)
        );

        let mut castle_moves = Vec::new();
        for m in mc.quiets_iter() {
            if m.move_type() == types::move_type::MoveType::Castle {
                castle_moves.push(format!("{}", m));
            }
        }

        println!("Castling moves generated: {:?}", castle_moves);
        println!("Expected: Only e8g8 (kingside), NOT e8c8 (queenside is illegal)");
    }

    #[test]
    fn deep_dive_perft_after_a2a3() {
        let mut pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/P1N2Q1p/1P1BBPPP/R3K2R b KQkq - 0 2",
        )
        .unwrap();

        println!("\n=== Deep Dive: Perft after a2a3 ===");

        // Do perft divide at depth 1
        let mut collector = MoveCollector::new();
        pos.generate_legal_moves(&mut collector);

        println!(
            "Move count at depth 1: {}",
            collector.capture_count + collector.quiet_count
        );

        // Now do perft(1) for each move to see which gives wrong counts
        let mut total = 0u64;
        println!("\nPer-move breakdown:");

        for m in collector.captures_iter() {
            let move_str = format!("{}", m);
            let undo = pos.make_move(m);
            let count = pos.perft(1);
            pos.unmake_move(m, undo);
            println!("  {} (capture): {}", move_str, count);
            total += count;
        }

        for m in collector.quiets_iter() {
            let move_str = format!("{}", m);
            let undo = pos.make_move(m);
            let count = pos.perft(1);
            pos.unmake_move(m, undo);

            // Flag suspicious moves
            if m.move_type() == types::move_type::MoveType::Castle && count > 50 {
                println!("  {} (CASTLE - SUSPICIOUS?): {}", move_str, count);
            } else {
                println!("  {} (quiet): {}", move_str, count);
            }
            total += count;
        }

        println!("\nTotal: {} (expected: 2134)", total);
    }
}
#[cfg(test)]
mod attack_detection_debug {
    use crate::position::Position;
    use types::piece_type::{Color, Piece};

    #[test]
    fn test_queen_attacks_from_f3() {
        // Simplified position: Queen on f3, empty d8
        let pos = Position::from_fen("3k4/8/8/8/8/5Q2/8/4K3 w - - 0 1").unwrap();

        println!("\n=== Testing Queen Attacks from f3 ===");
        println!("Queen on f3, Black king on d8");

        // Can the queen on f3 attack d8?
        let d8_attacked = pos.is_square_attacked(59, Color::White, None);
        println!("d8 (59) attacked by White Queen? {}", d8_attacked);

        // The queen should be able to attack d8 along the diagonal f3-e4-d5-c6-b7-a8
        // or... wait, that doesn't reach d8.
        // Along ranks/files: f3 can go to f8, then d8... no, that's two moves.
        // Actually, f3 cannot directly attack d8!

        println!("Expected: false (Queen on f3 cannot reach d8)");
    }

    #[test]
    fn find_what_attacks_d8_in_kiwipete() {
        // Full position after a2a3
        let pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/P1N2Q1p/1P1BBPPP/R3K2R b KQkq - 0 2",
        )
        .unwrap();

        println!("\n=== Finding what attacks d8 after a2a3 ===");

        // Check all White pieces that could potentially attack d8 (square 59)
        let d8 = 59;

        println!("Checking if d8 is attacked by each White piece type:");

        // Get white pieces
        let white_pieces = pos.colors[Color::White as usize];

        // Check each piece type
        for piece in [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ] {
            let pieces_bb = pos.pieces[piece as usize] & white_pieces;
            if pieces_bb.0 != 0 {
                println!("\n{:?}s at:", piece);
                let mut bb = pieces_bb.0;
                while bb != 0 {
                    let sq = bb.trailing_zeros() as usize;
                    bb &= bb - 1;
                    let file = sq % 8;
                    let rank = sq / 8;
                    let sq_name = format!("{}{}", (b'a' + file as u8) as char, rank + 1);
                    println!("  {} (square {})", sq_name, sq);
                }
            }
        }

        // Now manually check if White Queen from f3 can attack d8
        println!("\nManual check: Can White Queen on f3 attack d8?");
        let d8_attacked = pos.is_square_attacked(d8, Color::White, None);
        println!("Result: {}", d8_attacked);

        // Also check c8
        println!("\nManual check: Can any White piece attack c8?");
        let c8_attacked = pos.is_square_attacked(58, Color::White, None);
        println!("Result: {}", c8_attacked);
    }

    #[test]
    fn verify_queenside_castle_legality() {
        // Let's verify with a reference: in Kiwipete after a2a3,
        // can Black castle queenside?
        let mut pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/P1N2Q1p/1P1BBPPP/R3K2R b KQkq - 0 2",
        )
        .unwrap();

        println!("\n=== Verifying Queenside Castle Legality ===");

        // According to Stockfish, Black CANNOT castle queenside here
        // Let's figure out why

        // Check if king is in check
        let king_bb = pos.pieces[Piece::King as usize] & pos.colors[Color::Black as usize];
        let king_sq = king_bb.0.trailing_zeros() as usize;
        println!("Black king at square {}", king_sq);

        let in_check = pos.is_square_attacked(king_sq, Color::White, None);
        println!("King in check? {}", in_check);

        // Check castling path
        println!("\nChecking squares for queenside castle (e8-c8):");
        println!(
            "  e8 (60) attacked? {}",
            pos.is_square_attacked(60, Color::White, None)
        );
        println!(
            "  d8 (59) attacked? {}",
            pos.is_square_attacked(59, Color::White, None)
        );
        println!(
            "  c8 (58) attacked? {}",
            pos.is_square_attacked(58, Color::White, None)
        );

        // Check if squares are empty
        let blockers = pos.colors[0].0 | pos.colors[1].0;
        println!("\nChecking if squares are empty:");
        println!("  d8 (59) empty? {}", (blockers >> 59) & 1 == 0);
        println!("  c8 (58) empty? {}", (blockers >> 58) & 1 == 0);
        println!("  b8 (57) empty? {}", (blockers >> 57) & 1 == 0);

        // Try making the castle move and see if it's legal
        use types::move_collector::MoveCollector;
        let mut mc = MoveCollector::new();
        pos.generate_legal_moves(&mut mc);

        let mut found_queenside = false;
        for m in mc.quiets_iter() {
            if m.move_type() == types::move_type::MoveType::Castle && m.to() == 58 {
                found_queenside = true;
                println!("\n✗ ERROR: Queenside castle (e8c8) was generated as legal!");

                // Test if it's actually legal
                let is_legal = pos.is_legal_move(m);
                println!("  is_legal_move check: {}", is_legal);

                // Make the move and check if king ends up in check
                let undo = pos.make_move(m);
                let king_bb = pos.pieces[Piece::King as usize] & pos.colors[Color::Black as usize];
                let new_king_sq = king_bb.0.trailing_zeros() as usize;
                let king_in_check_after = pos.is_square_attacked(new_king_sq, Color::White, None);
                println!(
                    "  After castling, king at {} in check? {}",
                    new_king_sq, king_in_check_after
                );
                pos.unmake_move(m, undo);
            }
        }

        if !found_queenside {
            println!("\n✓ Correct: Queenside castle not generated");
        }
    }
}
