#![allow(dead_code)]

const FILE_A: u64 = 0x0101010101010101;
const FILE_H: u64 = 0x8080808080808080;

pub fn init_king_moves() -> Box<[u64; 64]> {
    let mut moves = [0u64; 64];

    for sq in 0..64 {
        let bb = 1u64 << sq;
        let mut attacks = 0u64;

        let not_a = !FILE_A;
        let not_h = !FILE_H;

        if bb & not_a != 0 {
            attacks |= bb >> 1;
            attacks |= (bb >> 9) & !0;
            attacks |= (bb << 7) & !0;
        }
        if bb & not_h != 0 {
            attacks |= bb << 1;
            attacks |= (bb >> 7) & !0;
            attacks |= (bb << 9) & !0;
        }

        attacks |= bb >> 8;
        attacks |= bb << 8;

        moves[sq] = attacks;
    }

    Box::new(moves)
}

#[cfg(test)]
mod tests {
    use magician::prelude::{notation_to_index, print_board};

    use super::*;

    #[test]
    fn test_king_moves_e6() {
        let king_moves = init_king_moves();
        let idx = notation_to_index("d4") as usize;
        let moves = king_moves[idx];
        println!("King moves from e6:");
        print_board(moves);
    }
}
