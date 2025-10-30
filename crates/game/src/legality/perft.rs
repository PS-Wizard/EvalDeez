use crate::position::Position;
use types::move_collector::MoveCollector;

impl Position {
    /// Perft (Performance Test) - counts leaf nodes at a given depth
    pub fn perft(&mut self, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut collector = MoveCollector::new();
        self.generate_legal_moves(&mut collector);

        if depth == 1 {
            return (collector.capture_count + collector.quiet_count) as u64;
        }

        let mut nodes = 0u64;

        // Process captures first
        for m in collector.captures_iter() {
            let undo = self.make_move(m);
            nodes += self.perft(depth - 1);
            self.unmake_move(m, undo);
        }

        // Then process quiet moves
        for m in collector.quiets_iter() {
            let undo = self.make_move(m);
            nodes += self.perft(depth - 1);
            self.unmake_move(m, undo);
        }

        nodes
    }

    /// Perft divide - shows move-by-move breakdown
    pub fn perft_divide(&mut self, depth: u8) {
        let mut collector = MoveCollector::new();
        self.generate_legal_moves(&mut collector);

        let mut total = 0u64;

        println!("Captures:");
        for m in collector.captures_iter() {
            let undo = self.make_move(m);
            let count = if depth <= 1 { 1 } else { self.perft(depth - 1) };
            self.unmake_move(m, undo);

            println!("{}: {}", m, count);
            total += count;
        }

        println!("\nQuiet moves:");
        for m in collector.quiets_iter() {
            let undo = self.make_move(m);
            let count = if depth <= 1 { 1 } else { self.perft(depth - 1) };
            self.unmake_move(m, undo);

            println!("{}: {}", m, count);
            total += count;
        }

        println!("\nTotal: {}", total);
    }
}

#[cfg(test)]
mod perft_tests {
    use crate::position::Position;
    use std::time::Instant;

    struct PerftTest {
        name: &'static str,
        fen: &'static str,
        depths: &'static [(u8, u64)],
    }

    const PERFT_SUITE: &[PerftTest] = &[
        PerftTest {
            name: "Starting Position",
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            depths: &[(1, 20), (2, 400), (3, 8_902), (4, 197_281), (5, 4_865_609)],
        },
        PerftTest {
            name: "Kiwipete",
            fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            depths: &[
                (1, 48),
                (2, 2_039),
                (3, 97_862),
                (4, 4_085_603),
                (5, 193_690_690),
            ],
        },
        PerftTest {
            name: "Position 3",
            fen: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
            depths: &[
                (1, 14),
                (2, 191),
                (3, 2_812),
                (4, 43_238),
                (5, 674_624),
                (6, 11_030_083),
            ],
        },
        PerftTest {
            name: "Position 4",
            fen: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            depths: &[(1, 6), (2, 264), (3, 9_467), (4, 422_333), (5, 15_833_292)],
        },
        PerftTest {
            name: "Position 5",
            fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            depths: &[
                (1, 44),
                (2, 1_486),
                (3, 62_379),
                (4, 2_103_487),
                (5, 89_941_194),
            ],
        },
        PerftTest {
            name: "Position 6",
            fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            depths: &[
                (1, 46),
                (2, 2_079),
                (3, 89_890),
                (4, 3_894_594),
                (5, 164_075_551),
            ],
        },
    ];

    #[test]
    fn perft_correctness() {
        println!("\n╔════════════════════════════════════════════╗");
        println!("║         PERFT CORRECTNESS TESTS            ║");
        println!("╚════════════════════════════════════════════╝\n");

        for test in PERFT_SUITE {
            println!("Position: {}", test.name);
            let mut pos = Position::from_fen(test.fen).unwrap();

            for &(depth, expected) in test.depths {
                let start = Instant::now();
                let nodes = pos.perft(depth);
                let elapsed = start.elapsed().as_secs_f64();

                assert_eq!(
                    nodes, expected,
                    "{} depth {} failed: got {}, expected {}",
                    test.name, depth, nodes, expected
                );

                let nps = if elapsed > 0.0 {
                    (nodes as f64 / elapsed) as u64
                } else {
                    0
                };
                println!(
                    "  ✓ Depth {}: {} nodes in {:.3}s ({} nps)",
                    depth,
                    format_num(nodes),
                    elapsed,
                    format_num(nps)
                );
            }
            println!();
        }
    }

    #[test]
    #[ignore]
    fn perft_stress_test() {
        println!("\n╔════════════════════════════════════════════╗");
        println!("║          PERFT STRESS TEST                 ║");
        println!("╚════════════════════════════════════════════╝\n");

        let test_positions = [
            (
                "Starting",
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                5,
            ),
            (
                "Kiwipete",
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
                5,
            ),
            ("Position 3", "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 6),
            (
                "Position 4",
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
                5,
            ),
        ];

        let mut total_nodes = 0u64;
        let mut total_time = 0.0;

        for (name, fen, depth) in test_positions {
            println!("Testing: {} (depth {})", name, depth);
            let mut pos = Position::from_fen(fen).unwrap();

            let start = Instant::now();
            let nodes = pos.perft(depth);
            let elapsed = start.elapsed().as_secs_f64();

            total_nodes += nodes;
            total_time += elapsed;

            let nps = (nodes as f64 / elapsed) as u64;
            println!("  {} nodes in {:.2}s", format_num(nodes), elapsed);
            println!("  {} nodes/sec\n", format_num(nps));
        }

        let avg_nps = (total_nodes as f64 / total_time) as u64;
        println!("╔════════════════════════════════════════════╗");
        println!("║              FINAL RESULTS                 ║");
        println!("╠════════════════════════════════════════════╣");
        println!("║ Total nodes:  {:>28} ║", format_num(total_nodes));
        println!("║ Total time:   {:>24.2}s ║", total_time);
        println!("║ Average NPS:  {:>28} ║", format_num(avg_nps));
        println!("╚════════════════════════════════════════════╝");
    }

    #[test]
    #[ignore]
    fn divide_debug() {
        println!("\n=== Perft Divide Debug ===");
        let mut pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        )
        .unwrap();
        pos.perft_divide(4);
    }

    fn format_num(n: u64) -> String {
        n.to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(|x| std::str::from_utf8(x).unwrap())
            .collect::<Vec<_>>()
            .join(",")
    }
}
