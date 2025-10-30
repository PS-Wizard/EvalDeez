use types::move_collector::MoveCollector;

use crate::position::Position;

mod bishops;
mod kings;
mod knights;
mod pawns;
mod queens;
mod rooks;

impl Position {
    pub fn generate_pseudo_legal_moves(&self, collector: &mut MoveCollector) {
        collector.clear();
        self.generate_pawn_moves(collector);
        self.generate_knight_moves(collector);
        self.generate_bishop_moves(collector);
        self.generate_rook_moves(collector);
        self.generate_queen_moves(collector);
        self.generate_king_moves(collector);
    }
}
