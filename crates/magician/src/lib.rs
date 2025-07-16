use std::sync::OnceLock;

use attacks::build_attack_table_for_square;
use rook::{blockers::rook_occupancy_mask, rook_attacks::rook_attacks_from};
use utils::load_magics_bin;

mod attacks;
mod bishop;
mod magic;
mod rook;
mod utils;

static ROOK_ATTACK_TABLES: OnceLock<Vec<Vec<u64>>> = OnceLock::new();
static ROOK_MAGICS_SHIFTS: OnceLock<Vec<(u64, u8)>> = OnceLock::new();

pub fn init_rook_attacks() {
    let magics = load_magics_bin("rook_magics.bin").expect("Failed to load rook magics");
    ROOK_MAGICS_SHIFTS.set(magics).ok();
    let mut tables = Vec::with_capacity(64);
    for square in 0..64 {
        let (magic, _) = ROOK_MAGICS_SHIFTS.get().unwrap()[square];
        let mask = rook_occupancy_mask(square as u8);
        let table = build_attack_table_for_square(square as u8, mask, magic, rook_attacks_from);
        tables.push(table);
    }
    ROOK_ATTACK_TABLES.set(tables).ok();
}
pub fn get_rook_attacks(square: u8, blockers: u64) -> u64 {
    let magics = ROOK_MAGICS_SHIFTS
        .get()
        .expect("ROOK magics not initialized");
    let tables = ROOK_ATTACK_TABLES
        .get()
        .expect("ROOK attack tables not initialized");

    let (magic, shift) = magics[square as usize];
    let index = ((blockers.wrapping_mul(magic)) >> shift) as usize;

    tables[square as usize][index]
}
#[cfg(test)]
mod tests {
    use std::time::Instant;

    use rand::{Rng, rng};

    use crate::{
        get_rook_attacks, init_rook_attacks,
        rook::blockers::rook_occupancy_mask,
        utils::{blockers_from_squares, enumerate_blocker_configs, notation_to_index, print_board},
    };

    #[test]
    pub fn test_get_rook_attacks() {
        init_rook_attacks();
        let square = notation_to_index("e4");
        let blockers = blockers_from_squares(&["e8", "c4", "e3"]);
        let attacks = get_rook_attacks(square, blockers);
        print_board(attacks);
        println!();

        let square = notation_to_index("a1");
        let blockers = 0u64;
        let attacks = get_rook_attacks(square, blockers);
        print_board(attacks);
        println!();

        let square = notation_to_index("a1");
        let blockers = blockers_from_squares(&["g1", "a6"]);
        let attacks = get_rook_attacks(square, blockers);
        print_board(attacks);
        println!();
    }

    #[test]
    fn test_rook_speed() {
        init_rook_attacks();

        let square = notation_to_index("e4");
        let mask = rook_occupancy_mask(square);
        let blocker_configs = enumerate_blocker_configs(mask);
        let mut rng = rand::rng();

        // Test 1: Lookup for all blocker configurations
        let start = Instant::now();
        for &blockers in &blocker_configs {
            let _attack = get_rook_attacks(square, blockers);
        }
        let elapsed = start.elapsed();
        println!(
            "Rook attack lookup on square e4 for {} blocker configs took {:?}",
            blocker_configs.len(),
            elapsed
        );

        // Test 2: Lookup for 1,000,000 random blockers
        let n = 1_000_000;
        let start = Instant::now();
        for _ in 0..n {
            // Sample from precomputed blockers instead of generating new ones
            let blockers = blocker_configs[rng.random_range(0..blocker_configs.len())];
            let _attack = get_rook_attacks(square, blockers);
        }
        let elapsed_rand = start.elapsed();
        println!(
            "Rook attack lookup {} random blockers took {:?}",
            n, elapsed_rand
        );
    }
}
