#![allow(dead_code)]
pub fn bishop_occupancy_mask(square: u8) -> u64 {
    let rank = square / 8;
    let file = square % 8;
    let mut mask = 0u64;

    // Directions: top-right, top-left, bottom-right, bottom-left
    for (dr, df) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
        let mut r = rank as i8 + dr;
        let mut f = file as i8 + df;

        while (1..7).contains(&r) && (1..7).contains(&f) {
            mask |= 1u64 << (r as u8 * 8 + f as u8);
            r += dr;
            f += df;
        }
    }

    mask
}

#[cfg(test)]
mod test_bishop {
    use super::bishop_occupancy_mask;
    use crate::utils::{
        enumerate_blocker_configs, notation_to_index, print_board, write_occupancies_to_bin,
    };

    #[test]
    fn test_bishop_occupancy() {
        println!("For E4:");
        let square = notation_to_index("e4");
        let mask = bishop_occupancy_mask(square);
        print_board(mask);

        println!("For a1:");
        let square = notation_to_index("a1");
        let mask = bishop_occupancy_mask(square);
        print_board(mask);
    }

    #[test]
    fn test_enumerate_bishop_blockers() {
        let square = notation_to_index("e4");
        let mask = bishop_occupancy_mask(square);
        println!("Occupancy mask for bishop @ e4:");
        print_board(mask);

        let blockers = enumerate_blocker_configs(mask);
        for b in &blockers {
            println!();
            print_board(*b);
            println!();
        }

        let expected = 2u64.pow(mask.count_ones());
        assert_eq!(
            blockers.len() as u64,
            expected,
            "Expected {} blocker variations, got {}",
            expected,
            blockers.len()
        );
    }

    #[test]
    #[ignore = "computationally heavier"]
    fn test_write_bishop_occupancies() {
        let mut entries = Vec::with_capacity(64);
        for square in 0..64 {
            let occupancy_mask = bishop_occupancy_mask(square);
            entries.push(occupancy_mask);
        }
        write_occupancies_to_bin("bishop_occupancies.bin", &entries).unwrap();
    }
}
