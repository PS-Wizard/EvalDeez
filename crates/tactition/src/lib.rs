mod king_moves;
mod knight_moves;

use king_moves::init_king_moves;
use knight_moves::init_knight_moves;
use std::sync::OnceLock;

static KING_ATTACKS: OnceLock<Box<[u64; 64]>> = OnceLock::new();
static KNIGHT_ATTACKS: OnceLock<Box<[u64; 64]>> = OnceLock::new();

pub fn init_precalculated_tables() {
    KING_ATTACKS.set(init_king_moves()).ok();
    KNIGHT_ATTACKS.set(init_knight_moves()).ok();
}

pub fn get_king_attacks(from: u8, blockers: u64) -> u64 {
    let table = KING_ATTACKS.get().expect("KING table not initialized");
    table[from as usize] & !blockers
}

pub fn get_knight_attacks(from: u8, blockers: u64) -> u64 {
    let table = KNIGHT_ATTACKS.get().expect("KNIGHT table not initialized");
    table[from as usize] & !blockers
}

#[cfg(test)]
mod test {
    use super::*;
    use magician::prelude::*;

    #[test]
    fn test_attacks_with_blockers() {
        init_magician();
        init_precalculated_tables();

        let from = notation_to_index("e7");

        let blocker_squares = &["d6", "f6"];
        let blockers = blockers_from_squares(blocker_squares);

        println!("Blockers:");
        print_board(blockers);

        println!("King attacks from e6:");
        print_board(get_king_attacks(from, blockers));

        println!("Knight attacks from e6:");
        print_board(get_knight_attacks(from, blockers));

        println!("Bishop attacks from e6:");
        print_board(get_bishop_attacks(from, blockers));
    }
}
