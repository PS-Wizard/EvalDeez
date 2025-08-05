#![allow(dead_code)]

const FILE_A: u64 = 0x0101010101010101;
const FILE_H: u64 = 0x8080808080808080;

pub fn generate_pawn_moves(
    pawn_pos: u8,
    is_white: bool,
    blockers: u64,
    enemy_pieces: u64,
    en_passant_square: Option<u8>,
) -> u64 {
    let bb = 1u64 << pawn_pos;
    let mut moves = 0u64;

    if is_white {
        let one_step = bb << 8;
        if one_step & blockers == 0 {
            moves |= one_step;

            if pawn_pos / 8 == 1 {
                let two_step = bb << 16;
                if (one_step | two_step) & blockers == 0 {
                    moves |= two_step;
                }
            }
        }

        if bb & !FILE_A != 0 {
            let left = bb << 7;
            if left & enemy_pieces != 0 {
                moves |= left;
            }
        }

        if bb & !FILE_H != 0 {
            let right = bb << 9;
            if right & enemy_pieces != 0 {
                moves |= right;
            }
        }

        // en passant
        if let Some(ep_sq) = en_passant_square {
            let ep_bb = 1u64 << ep_sq;
            if (bb << 7 == ep_bb && bb & !FILE_A != 0) || (bb << 9 == ep_bb && bb & !FILE_H != 0) {
                moves |= ep_bb;
            }
        }
    } else {
        let one_step = bb >> 8;
        if one_step & blockers == 0 {
            moves |= one_step;

            // double push (only if on rank 7)
            if pawn_pos / 8 == 6 {
                let two_step = bb >> 16;
                if (one_step | two_step) & blockers == 0 {
                    moves |= two_step;
                }
            }
        }

        // captures
        if bb & !FILE_A != 0 {
            let left = bb >> 9;
            if left & enemy_pieces != 0 {
                moves |= left;
            }
        }
        if bb & !FILE_H != 0 {
            let right = bb >> 7;
            if right & enemy_pieces != 0 {
                moves |= right;
            }
        }

        if let Some(ep_sq) = en_passant_square {
            let ep_bb = 1u64 << ep_sq;
            if (bb >> 9 == ep_bb && bb & !FILE_A != 0) || (bb >> 7 == ep_bb && bb & !FILE_H != 0) {
                moves |= ep_bb;
            }
        }
    }

    moves
}
#[cfg(test)]
mod test {
    use magician::prelude::{blockers_from_squares, notation_to_index, print_board};

    use super::generate_pawn_moves;

    #[test]
    fn test_white_pawn_d4_capture_ep() {
        let pawn_sq = notation_to_index("d4");
        let enemy = blockers_from_squares(&["e5"]);
        let blockers = blockers_from_squares(&["d5"]);
        let ep = None;
        let moves = generate_pawn_moves(pawn_sq, true, blockers, enemy, ep);
        print_board(moves);
    }
}
