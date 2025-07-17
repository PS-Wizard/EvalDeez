#![allow(dead_code)]
use std::sync::OnceLock;

use attacks::build_attack_table_for_square;
use bishop::bishop_attacks::bishop_attacks_from;
use rook::rook_attacks::rook_attacks_from;
use utils::{load_magics_bin, load_occupancies_bin, magician_data_file};

mod attacks;
mod bishop;
mod magic;
pub mod prelude;
mod rook;
mod utils;

static ROOK_ATTACK_TABLES: OnceLock<Vec<Vec<u64>>> = OnceLock::new();
static ROOK_MAGICS_SHIFTS: OnceLock<Vec<(u64, u8)>> = OnceLock::new();
static ROOK_OCCUPANCIES: OnceLock<Vec<u64>> = OnceLock::new();

static BISHOP_ATTACK_TABLES: OnceLock<Vec<Vec<u64>>> = OnceLock::new();
static BISHOP_MAGICS_SHIFTS: OnceLock<Vec<(u64, u8)>> = OnceLock::new();
static BISHOP_OCCUPANCIES: OnceLock<Vec<u64>> = OnceLock::new();

pub fn init_magician() {
    init_rook_attacks();
    init_bishop_attacks();
}

fn init_bishop_attacks() {
    let magics = load_magics_bin(magician_data_file("bishop_magics.bin").to_str().unwrap())
        .expect("Failed to load bishop magics");
    let occupancies = load_occupancies_bin(
        magician_data_file("bishop_occupancies.bin")
            .to_str()
            .unwrap(),
    )
    .expect("Failed To Load Bishop Occupancies");

    BISHOP_MAGICS_SHIFTS.set(magics).ok();
    BISHOP_OCCUPANCIES.set(occupancies.clone()).ok();

    let mut tables = Vec::with_capacity(64);
    for square in 0..64 {
        let (magic, _) = BISHOP_MAGICS_SHIFTS.get().unwrap()[square];
        let mask = occupancies[square];
        let table = build_attack_table_for_square(square as u8, mask, magic, bishop_attacks_from);
        tables.push(table);
    }
    BISHOP_ATTACK_TABLES.set(tables).ok();
}

pub fn get_bishop_attacks(square: u8, blockers: u64) -> u64 {
    let magics = BISHOP_MAGICS_SHIFTS
        .get()
        .expect("BISHOP magics not initialized");
    let tables = BISHOP_ATTACK_TABLES
        .get()
        .expect("BISHOP attack tables not initialized");
    let occupancies = BISHOP_OCCUPANCIES
        .get()
        .expect("BISHOP occupancies not initialized");

    let (magic, shift) = magics[square as usize];
    let mask = occupancies[square as usize];
    let index = ((blockers & mask).wrapping_mul(magic) >> shift) as usize;

    tables[square as usize][index]
}

fn init_rook_attacks() {
    let magics = load_magics_bin(magician_data_file("rook_magics.bin").to_str().unwrap())
        .expect("Failed to load rook magics");
    let occupancies =
        load_occupancies_bin(magician_data_file("rook_occupancies.bin").to_str().unwrap())
            .expect("Failed to load rook occupancies");
    ROOK_MAGICS_SHIFTS.set(magics).ok();
    ROOK_OCCUPANCIES.set(occupancies.clone()).ok();

    let mut tables = Vec::with_capacity(64);
    for square in 0..64 {
        let (magic, _) = ROOK_MAGICS_SHIFTS.get().unwrap()[square];
        let mask = occupancies[square];
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
    let occupancies = ROOK_OCCUPANCIES
        .get()
        .expect("ROOK occupancies not initialized");

    let (magic, shift) = magics[square as usize];
    let mask = occupancies[square as usize];
    let index = ((blockers & mask).wrapping_mul(magic) >> shift) as usize;

    tables[square as usize][index]
}

pub fn get_queen_attacks(square: u8, blockers: u64) -> u64 {
    let rook_attacks = get_rook_attacks(square, blockers);
    let bishop_attacks = get_bishop_attacks(square, blockers);
    rook_attacks | bishop_attacks
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use rand::Rng;

    use crate::{
        bishop::blockers::bishop_occupancy_mask,
        get_bishop_attacks, get_queen_attacks, get_rook_attacks, init_bishop_attacks,
        init_magician, init_rook_attacks,
        rook::blockers::rook_occupancy_mask,
        utils::{blockers_from_squares, enumerate_blocker_configs, notation_to_index, print_board},
    };

    #[test]
    fn test_get_rook_attacks() {
        init_rook_attacks();
        let square = notation_to_index("d4");
        let blockers = blockers_from_squares(&[ "c4", "e4"]);
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
            let blockers = blocker_configs[rng.random_range(0..blocker_configs.len())];
            let _attack = get_rook_attacks(square, blockers);
        }
        let elapsed_rand = start.elapsed();
        println!(
            "Rook attack lookup {} random blockers took {:?}",
            n, elapsed_rand
        );
    }

    #[test]
    fn test_get_bishop_attacks() {
        init_bishop_attacks();
        let square = notation_to_index("e4");
        let blockers = blockers_from_squares(&[]);
        let attacks = get_bishop_attacks(square, blockers);
        print_board(attacks);
        println!();

        let square = notation_to_index("c3");
        let blockers = 0u64;
        let attacks = get_bishop_attacks(square, blockers);
        print_board(attacks);
        println!();

        let square = notation_to_index("f6");
        let blockers = blockers_from_squares(&["d4", "h8"]);
        let attacks = get_bishop_attacks(square, blockers);
        print_board(attacks);
        println!();
    }

    #[test]
    fn test_bishop_speed() {
        init_bishop_attacks();

        let square = notation_to_index("e4");
        let mask = bishop_occupancy_mask(square);
        let blocker_configs = enumerate_blocker_configs(mask);
        let mut rng = rand::rng();

        // Test 1: Lookup for all blocker configurations
        let start = Instant::now();
        for &blockers in &blocker_configs {
            let _attack = get_bishop_attacks(square, blockers);
        }
        let elapsed = start.elapsed();
        println!(
            "Bishop attack lookup on square e4 for {} blocker configs took {:?}",
            blocker_configs.len(),
            elapsed
        );

        let n = 1_000_000;
        let start = Instant::now();
        for _ in 0..n {
            let blockers = blocker_configs[rng.random_range(0..blocker_configs.len())];
            let _attack = get_bishop_attacks(square, blockers);
        }
        let elapsed_rand = start.elapsed();
        println!(
            "Bishop attack lookup {} random blockers took {:?}",
            n, elapsed_rand
        );
    }

    #[test]
    fn test_get_queen_attacks() {
        init_magician();
        let square = notation_to_index("e4");
        let blockers = 0u64;
        let attacks = get_queen_attacks(square, blockers);
        println!("Queen Attacks, No Blockers:");
        print_board(attacks);
        println!();

        let square = notation_to_index("a1");
        let blockers = blockers_from_squares(&["b2"]);
        let attacks = get_queen_attacks(square, blockers);
        print_board(attacks);

        let bishop_attacks = get_bishop_attacks(square, blockers);
        println!("bishop should be fine:");
        print_board(bishop_attacks);

        let rook_attacks = get_rook_attacks(square, blockers);
        println!("this rook shouldnt be ");
        print_board(rook_attacks);
        println!();

        let square = notation_to_index("d4");
        let blockers = blockers_from_squares(&["c4","e4"]);
        let attacks = get_queen_attacks(square, blockers);
        print_board(attacks);

        let bishop_attacks = get_bishop_attacks(square, blockers);
        println!("bishop shouldnt be fine:");
        print_board(bishop_attacks);

        let rook_attacks = get_rook_attacks(square, blockers);
        println!("this rook should be ");
        print_board(rook_attacks);
        println!();
    }

    #[test]
    fn test_queen_speed() {
        init_magician();

        let square = notation_to_index("e4");
        let mask = rook_occupancy_mask(square) | bishop_occupancy_mask(square);
        let blocker_configs = enumerate_blocker_configs(mask);
        let mut rng = rand::rng();

        // Test 1: Lookup for all blocker configurations
        let start = Instant::now();
        for &blockers in &blocker_configs {
            let _attack = get_queen_attacks(square, blockers);
        }
        let elapsed = start.elapsed();
        println!(
            "Queen attack lookup on square e4 for {} blocker configs took {:?}",
            blocker_configs.len(),
            elapsed
        );

        // Test 2: Lookup for 1,000,000 random blockers
        let n = 1_000_000;
        let start = Instant::now();
        for _ in 0..n {
            let blockers = blocker_configs[rng.random_range(0..blocker_configs.len())];
            let _attack = get_queen_attacks(square, blockers);
        }
        let elapsed_rand = start.elapsed();
        println!(
            "Queen attack lookup {} random blockers took {:?}",
            n, elapsed_rand
        );
    }
}
