pub fn rook_attacks_from(square: u8, blockers: u64) -> u64 {
    let rank = square / 8;
    let file = square % 8;
    let mut attacks = 0u64;

    // Up
    for r in (rank + 1)..8 {
        let sq = r * 8 + file;
        attacks |= 1u64 << sq;
        if (blockers >> sq) & 1 == 1 {
            break;
        }
    }

    // Down
    for r in (0..rank).rev() {
        let sq = r * 8 + file;
        attacks |= 1u64 << sq;
        if (blockers >> sq) & 1 == 1 {
            break;
        }
    }

    // Right
    for f in (file + 1)..8 {
        let sq = rank * 8 + f;
        attacks |= 1u64 << sq;
        if (blockers >> sq) & 1 == 1 {
            break;
        }
    }

    // Left
    for f in (0..file).rev() {
        let sq = rank * 8 + f;
        attacks |= 1u64 << sq;
        if (blockers >> sq) & 1 == 1 {
            break;
        }
    }

    attacks
}

#[cfg(test)]
mod test_rook {
    use crate::{
        rook::blockers::rook_occupancy_mask,
        utils::{enumerate_blocker_configs, notation_to_index, print_board},
    };

    use super::rook_attacks_from;

    #[test]
    fn test_attack_generation() {
        let possible_occupancy = rook_occupancy_mask(notation_to_index("e4"));
        let blocker_variations = enumerate_blocker_configs(possible_occupancy);
        for blockers in blocker_variations {
            println!("");
            println!("The Blocker:");
            print_board(blockers);
            let attacks = rook_attacks_from(notation_to_index("e4"), blockers);
            println!("The Attacks:");
            print_board(attacks);
            println!("")
        }
    }
}
