use std::collections::HashMap;
use std::hash::Hash;

use crate::enums::chess_error::ChessError;
use crate::enums::end_type::EndType;
use crate::{enums::chess_color::ChessColor, board_types::bitboard::Constants};

pub trait ChessBoardContract where Self: Sized + Eq + Hash {
    fn generate_moves(&self, turn: ChessColor, constants: &Constants) -> Result<Vec<(String, Self)>, ChessError>;
    fn check_game_end(&self, turn: ChessColor, board_history: &HashMap<Self, i32>, constants: &Constants) -> Result<EndType, ChessError>;
    fn get_value_of_pieces(&self, piece_values: [i32; 6]) -> i32;
    fn new_board() -> Self;
    fn board_ascii(&self, use_unicode: bool) -> String;
    fn board_to_fen(&self, turn: ChessColor) -> String;
}
