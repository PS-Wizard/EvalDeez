pub fn rook_occupancy_mask(square: u8) -> u64 {
    let rank = square / 8;
    let file = square % 8;

    let mut mask = 0u64;

    // Directions: up, down (same file)
    for r in (rank + 1)..7 {
        mask |= 1u64 << (r * 8 + file);
    }
    for r in (1..rank).rev() {
        mask |= 1u64 << (r * 8 + file);
    }

    // Directions: left, right (same rank)
    for f in (file + 1)..7 {
        mask |= 1u64 << (rank * 8 + f);
    }
    for f in (1..file).rev() {
        mask |= 1u64 << (rank * 8 + f);
    }

    mask
}

#[cfg(test)]
mod test_rook {
    use crate::utils::{enumerate_blocker_configs, notation_to_index, print_board};

    use super::rook_occupancy_mask;

    #[test]
    fn test_rook_occupancy() {
        let possible_occupancies = rook_occupancy_mask(notation_to_index("e4"));
        print_board(possible_occupancies);
    }

    #[test]
    fn test_enumerate_blocker_configs() {
        let possible_occupancy = rook_occupancy_mask(notation_to_index("e4"));
        println!("For: ");
        print_board(possible_occupancy);
        let blocker_variations = enumerate_blocker_configs(possible_occupancy);
        for variation in &blocker_variations {
            println!("");
            print_board(*variation);
            println!("");
        }
        assert_eq!(
            blocker_variations.len(),
            1024,
            "There should be 1024 total blocker variations for a rook on e4, excluding edges there are total 10 squares, so its just 2^10"
        );
    }
}
