use rand::Rng;

use crate::utils::enumerate_blocker_configs;

fn generate_sparse_u64(min_bits: u32, max_bits: u32) -> u64 {
    let mut rng = rand::rng();
    let num_bits = rng.random_range(min_bits..=max_bits);
    let mut candidate = 0;
    let mut set_bits = 0;
    while set_bits < num_bits {
        let bit = 1 << rng.random_range(0..64);
        if candidate & bit == 0 {
            candidate |= bit;
            set_bits += 1;
        }
    }
    candidate
}

pub fn find_magics_number(square: u8, mask: &u64) -> u64 {
    println!("Finding magic for square: {}", square);
    let relevant_bits = mask.count_ones();
    let blocker_configs = enumerate_blocker_configs(*mask);
    println!(
        "Generated: {} blocker configurations, expected: {}",
        blocker_configs.len(),
        1 << relevant_bits
    );
    assert_eq!(
        blocker_configs.len(),
        1 << relevant_bits as usize,
        "Incorrect number of blocker configurations for square {}",
        square
    );
    let shift = 64 - relevant_bits;
    let table_size = 1 << relevant_bits;
    let index_mask = table_size - 1;

    'magic_search: for _ in 0..1_000_000 {
        let magic_candidate = generate_sparse_u64(6, 10);
        let mut used_indices = vec![false; table_size];

        for &blockers in &blocker_configs {
            let index = (blockers.wrapping_mul(magic_candidate) >> shift) as usize;
            let index = index & index_mask;

            if used_indices[index] {
                continue 'magic_search;
            }
            used_indices[index] = true;
        }

        println!("Found Magic For: {square}: {magic_candidate}");
        return magic_candidate;
    }
    panic!("No magic number found for square {}", square);
}

// NOTE:
// First 8 bytes magic number, next 1 byte shift

#[cfg(test)]
mod test_magics {
    use crate::{
        bishop::blockers::bishop_occupancy_mask, magic::find_magics_number,
        rook::blockers::rook_occupancy_mask, utils::write_magics_to_bin,
    };

    #[test]
    #[ignore = "computationally heavier"]
    fn test_generate_rook_magics() {
        let mut entries = Vec::with_capacity(64);
        for square in 0..64 {
            println!("\n=== Processing square {} ===", square);
            let occupancy_mask = rook_occupancy_mask(square);
            let shift = 64 - occupancy_mask.count_ones();
            let magic_number = find_magics_number(square, &occupancy_mask);
            println!("Found Magic Number for square {}: {}", square, magic_number);
            entries.push((magic_number, shift as u8));
        }
        write_magics_to_bin("rook_magics.bin", &entries).unwrap();
    }

    #[test]
    #[ignore = "computationally heavier"]
    fn test_generate_bishop_magics() {
        let mut entries = Vec::with_capacity(64);
        for square in 0..64 {
            println!("\n=== Processing square {} ===", square);
            let occupancy_mask = bishop_occupancy_mask(square);
            let shift = 64 - occupancy_mask.count_ones();
            let magic_number = find_magics_number(square, &occupancy_mask);
            println!("Found Magic Number for square {}: {}", square, magic_number);
            entries.push((magic_number, shift as u8));
        }
        write_magics_to_bin("bishop_magics.bin", &entries).unwrap();
    }
}
