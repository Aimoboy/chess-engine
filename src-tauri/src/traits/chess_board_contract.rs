use crate::enums::chess_error::ChessError;
use crate::enums::end_type::EndType;
use crate::{enums::piece_color::PieceColor, board_types::bitboard::Constants};

pub trait ChessBoardContract where Self: Sized {
    fn generate_moves(&self, prev_board: Option<&Self>, turn: PieceColor, constants: &Constants) -> Result<Vec<(String, Self)>, ChessError>;
    fn check_game_end(&self, prev_board: Option<&Self>, turn: PieceColor, constants: &Constants) -> Result<EndType, ChessError>;
    fn get_value_of_pieces(&self, piece_values: [i32; 6]) -> i32;
    fn new_board() -> Self;
    fn board_ascii(&self, use_unicode: bool) -> String;
}
