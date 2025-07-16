use crate::utils::enumerate_blocker_configs;

pub fn build_attack_table_for_square(
    square: u8,
    mask: u64,
    magic: u64,
    attacks_fn: impl Fn(u8, u64) -> u64,
) -> Vec<u64> {
    let blocker_configs = enumerate_blocker_configs(mask);
    let relevant_bits = mask.count_ones();
    let table_size = 1 << relevant_bits;
    let mut attack_table = vec![0u64; table_size as usize];

    for blockers in blocker_configs {
        let index = ((blockers.wrapping_mul(magic)) >> (64 - relevant_bits)) as usize;
        attack_table[index] = attacks_fn(square, blockers);
    }

    attack_table
}

#[cfg(test)]
mod test_attacks {
    use crate::{
        attacks::build_attack_table_for_square,
        rook::{blockers::rook_occupancy_mask, rook_attacks::rook_attacks_from},
        utils::{blockers_from_squares, load_magics_bin, notation_to_index, print_board},
    };

    #[test]
    fn test_build_attack_table_and_lookup() {
        let magics = load_magics_bin("magics.bin").expect("Failed to load magics.bin");
        let square = notation_to_index("e4");

        let (magic, shift) = magics[square as usize];
        let mask = rook_occupancy_mask(square);

        // Build the attack table for this square
        let attack_table = build_attack_table_for_square(square, mask, magic, rook_attacks_from);

        let blockers = blockers_from_squares(&["e8", "c4", "e3"]);
        let index = ((blockers.wrapping_mul(magic)) >> shift) as usize;

        let attack = attack_table[index];
        print_board(attack);
    }
}
