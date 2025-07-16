use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn print_board(bb: u64) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = rank * 8 + file;
            let bit = (bb >> square) & 1;
            if bit == 1 {
                print!(" X ");
            } else {
                print!(" . ");
            }
        }
        println!();
    }
}

pub fn notation_to_index(notation: &str) -> u8 {
    let bytes = notation.as_bytes();
    let file = bytes[0];
    let rank = bytes[1];

    let file_idx = file - b'a';
    let rank_idx = rank - b'1';

    rank_idx * 8 + file_idx
}

pub fn enumerate_blocker_configs(mask: u64) -> Vec<u64> {
    let mut relevant_bits = Vec::new();
    for i in 0..64 {
        if (mask >> i) & 1 == 1 {
            relevant_bits.push(i);
        }
    }

    let num_bits = relevant_bits.len();
    let num_configs = 1 << num_bits; // 2^n
    let mut configs = Vec::with_capacity(num_configs);

    for i in 0..num_configs {
        let mut blocker = 0u64;
        for j in 0..num_bits {
            if (i >> j) & 1 == 1 {
                blocker |= 1u64 << relevant_bits[j];
            }
        }
        configs.push(blocker);
    }

    configs
}

pub fn load_magics_bin(filename: &str) -> std::io::Result<Vec<(u64, u8)>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut entries = Vec::new();
    let mut buf = [0u8; 9]; // 8 bytes magic + 1 byte shift

    while reader.read_exact(&mut buf).is_ok() {
        let magic = u64::from_le_bytes(buf[0..8].try_into().unwrap());
        let shift = buf[8];
        entries.push((magic, shift));
    }
    Ok(entries)
}

pub fn blockers_from_squares(squares: &[&str]) -> u64 {
    squares
        .iter()
        .fold(0u64, |acc, &sq| acc | (1u64 << notation_to_index(sq)))
}
