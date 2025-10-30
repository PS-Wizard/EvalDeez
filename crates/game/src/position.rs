use std::arch::x86_64::_pext_u64;

use raw::{
    BISHOP_ATTACKS, BISHOP_MASKS, KING_ATTACKS, KNIGHT_ATTACKS, PAWN_ATTACKS, ROOK_ATTACKS,
    ROOK_MASKS,
};
use types::{
    bitboard::Bitboard,
    piece_type::{Color, Piece, Piece::*},
    rights::CastleRights,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ColoredPiece {
    piece: Piece,
    color: Color,
}

impl ColoredPiece {
    #[inline(always)]
    pub const fn new(piece: Piece, color: Color) -> Self {
        Self { piece, color }
    }

    #[inline(always)]
    pub const fn piece(self) -> Piece {
        self.piece
    }

    #[inline(always)]
    pub const fn color(self) -> Color {
        self.color
    }
}

/// A struct that represents the entire game state. Uses bitboards to represent pieces, 6 total,
/// one for each piece type, uses colors to distinguish between the piece of each side. Contains
/// a piece square mapping for O(1) identification of a piece.
#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub pieces: [Bitboard; 6],
    pub colors: [Bitboard; 2],
    // Mailbox
    pub piece_map: [Option<ColoredPiece>; 64],
    pub side_to_move: Color,
    pub castling_rights: CastleRights,
    pub en_passant: Option<u8>,
    pub halfmove_clock: u8,
    pub fullmove_clock: u16,
    // Zobrist Hash
    pub hash: u64,
}

impl Position {
    /// Returns a new position struct set to the initial game state.
    #[inline(always)]
    pub fn new() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("Starting position FEN should always be valid")
    }

    /// Returns a new position struct set to the provided fen
    #[inline(always)]
    pub fn new_from_fen(fen: &str) -> Self {
        Self::from_fen(fen).expect("Invalid FEN string")
    }

    /// Returns all friendly pieces based on the side to move.
    #[inline(always)]
    pub fn us(&self) -> Bitboard {
        self.colors[self.side_to_move]
    }

    /// Returns all enemy pieces based on the side to move.
    #[inline(always)]
    pub fn them(&self) -> Bitboard {
        self.colors[self.side_to_move.flip()]
    }

    /// Returns the bitboard of a specific piece of the current player
    #[inline(always)]
    pub fn friendly(&self, piece: Piece) -> Bitboard {
        self.pieces[piece] & self.us()
    }

    /// Returns the bitboard of a specific piece of the opponent
    #[inline(always)]
    pub fn enemy(&self, piece: Piece) -> Bitboard {
        self.pieces[piece] & self.them()
    }

    /// Removes piece at a given square
    #[inline(always)]
    pub fn remove_piece(&mut self, sq: usize) {
        if let Some(cp) = self.piece_map[sq] {
            self.pieces[cp.piece()].remove_bit(sq);
            self.colors[cp.color()].remove_bit(sq);
            self.piece_map[sq] = None;
        }
    }

    /// Adds a piece at a given square
    #[inline(always)]
    pub fn add_piece(&mut self, sq: usize, piece: Piece, color: Color) {
        self.pieces[piece].set_bit(sq);
        self.colors[color].set_bit(sq);
        self.piece_map[sq] = Some(ColoredPiece::new(piece, color));
    }

    /// Given a square, returns the piece at that square
    #[inline(always)]
    pub fn piece_at(&self, sq: usize) -> Option<ColoredPiece> {
        self.piece_map[sq]
    }

    #[inline(always)]
    pub fn is_square_attacked(&self, sq: usize, by_side: Color, blockers: Option<u64>) -> bool {
        let blockers = blockers.unwrap_or(self.colors[0].0 | self.colors[1].0);

        let enemy_color = by_side;
        let enemy_pieces = self.colors[enemy_color];

        // Pawn attacks (need to check from the defender's perspective)
        let defender_color = by_side.flip();
        if (PAWN_ATTACKS[defender_color as usize][sq] & self.pieces[Pawn].0 & enemy_pieces.0) != 0 {
            return true;
        }

        // Knight attacks
        if (KNIGHT_ATTACKS[sq] & self.pieces[Knight].0 & enemy_pieces.0) != 0 {
            return true;
        }

        // King attacks
        if (KING_ATTACKS[sq] & self.pieces[King].0 & enemy_pieces.0) != 0 {
            return true;
        }

        // Rook/Queen attacks
        let rook_idx = unsafe { _pext_u64(blockers, ROOK_MASKS[sq]) as usize };
        let rook_attacks = ROOK_ATTACKS[sq][rook_idx];
        if (rook_attacks & ((self.pieces[Rook].0 | self.pieces[Queen].0) & enemy_pieces.0)) != 0 {
            return true;
        }

        // Bishop/Queen attacks
        let bishop_idx = unsafe { _pext_u64(blockers, BISHOP_MASKS[sq]) as usize };
        let bishop_attacks = BISHOP_ATTACKS[sq][bishop_idx];
        if (bishop_attacks & ((self.pieces[Bishop].0 | self.pieces[Queen].0) & enemy_pieces.0)) != 0
        {
            return true;
        }

        false
    }
}
