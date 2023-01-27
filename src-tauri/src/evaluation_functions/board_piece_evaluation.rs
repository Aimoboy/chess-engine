use crate::enums::chess_error::ChessError;
use crate::traits::{
    chess_board_contract::ChessBoardContract
};

use crate::enums::{
    chess_color::ChessColor,
    end_type::EndType
};

use crate::board_types::bitboard::{
    Constants
};

const PIECE_VALUES: [i32; 6] = [100, 500, 300, 300, 900, 0];

pub fn board_piece_evaluation<T: ChessBoardContract>(board: &T, _: &Vec<T>, depth: i32, constants: &Constants) -> Result<i32, ChessError> {
    match board.check_game_end(ChessColor::White, constants)? {
        EndType::Checkmate(_) => {
            return Ok(<i32>::min_value() / 2 + depth);
        },
        EndType::Tie => {
            return Ok(0);
        },
        EndType::NoEnd => ()
    }

    match board.check_game_end(ChessColor::White, constants)? {
        EndType::Checkmate(_) => {
            return Ok(<i32>::max_value() / 2 - depth);
        },
        EndType::Tie => {
            return Ok(0);
        },
        EndType::NoEnd => ()
    }

    Ok(board.get_value_of_pieces(PIECE_VALUES))
}
