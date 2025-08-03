#![allow(dead_code)]

const FILE_A: u64 = 0x0101010101010101;
const FILE_AB: u64 = 0x0303030303030303;
const FILE_H: u64 = 0x8080808080808080;
const FILE_GH: u64 = 0xC0C0C0C0C0C0C0C0;

pub fn init_knight_moves() -> Box<[u64; 64]> {
    let mut moves = [0u64; 64];

    for sq in 0..64 {
        let bb = 1u64 << sq;
        let mut attacks = 0u64;

        // Pre-mask so we don’t wrap around
        if bb & !FILE_A != 0 {
            if bb & !FILE_AB != 0 {
                attacks |= bb >> 10; // 2 left, 1 down
                attacks |= bb << 6; // 2 left, 1 up
            }
            attacks |= bb >> 17; // 1 left, 2 down
            attacks |= bb << 15; // 1 left, 2 up
        }

        if bb & !FILE_H != 0 {
            if bb & !FILE_GH != 0 {
                attacks |= bb >> 6; // 2 right, 1 down
                attacks |= bb << 10; // 2 right, 1 up
            }
            attacks |= bb >> 15; // 1 right, 2 down
            attacks |= bb << 17; // 1 right, 2 up
        }

        moves[sq] = attacks;
    }

    Box::new(moves)
}

#[cfg(test)]
mod tests {
    use magician::prelude::{notation_to_index, print_board};

    use super::*;

    #[test]
    fn test_knight_moves_e6() {
        let knight_moves = init_knight_moves();
        let idx = notation_to_index("e6") as usize;
        let moves = knight_moves[idx];
        println!("Knight moves from e6:");
        print_board(moves);
    }
}
