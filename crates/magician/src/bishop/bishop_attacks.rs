
#![allow(dead_code)]
pub fn bishop_attacks_from(square: u8, blockers: u64) -> u64 {
    let rank = square / 8;
    let file = square % 8;
    let mut attacks = 0u64;

    // Directions: top-right, top-left, bottom-right, bottom-left
    for (dr, df) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
        let mut r = rank as i8 + dr;
        let mut f = file as i8 + df;

        while (0..8).contains(&r) && (0..8).contains(&f) {
            let idx = r as u8 * 8 + f as u8;
            attacks |= 1u64 << idx;
            if blockers & (1u64 << idx) != 0 {
                break;
            }
            r += dr;
            f += df;
        }
    }

    attacks
}

#[cfg(test)]
mod test_bishop {
    use crate::{
        bishop::{bishop_attacks::bishop_attacks_from, blockers::bishop_occupancy_mask},
        utils::{blockers_from_squares, enumerate_blocker_configs, notation_to_index, print_board},
    };

    #[test]
    fn test_bishop_attack_generation_single() {
        let blockers = blockers_from_squares(&["c4"]);
        println!("");
        println!("The Blocker:");
        print_board(blockers);
        let attacks = bishop_attacks_from(notation_to_index("d4"), blockers);
        println!("The Attacks:");
        print_board(attacks);
        println!("");
    }

    #[test]
    fn test_bishop_attack_generation() {
        let possible_occupancy = bishop_occupancy_mask(notation_to_index("e4"));
        let blocker_variations = enumerate_blocker_configs(possible_occupancy);
        for blockers in blocker_variations {
            println!("");
            println!("The Blocker:");
            print_board(blockers);
            let attacks = bishop_attacks_from(notation_to_index("e4"), blockers);
            println!("The Attacks:");
            print_board(attacks);
            println!("");
        }
    }
}
