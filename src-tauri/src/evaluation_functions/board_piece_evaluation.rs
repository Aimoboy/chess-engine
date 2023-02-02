use std::collections::HashMap;
use std::hash::Hash;

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
const TIE_VALUE: i32 = 20_000;

pub fn board_piece_evaluation<T: ChessBoardContract + Eq + Hash>(board: &T, board_history: &HashMap<T, i32>, turn: ChessColor, depth: i32, constants: &Constants) -> Result<i32, ChessError> {
    match board.check_game_end(turn, board_history, constants)? {
        EndType::WhiteWin => {
            return Ok(<i32>::max_value() / 2 - depth)
        },
        EndType::BlackWin => {
            return Ok(<i32>::min_value() / 2 + depth)
        },
        EndType::Tie => {
            return match turn {
                ChessColor::White => Ok(-TIE_VALUE),
                ChessColor::Black => Ok(TIE_VALUE)
            };
        },
        EndType::NoEnd => ()
    }

    Ok(board.get_value_of_pieces(PIECE_VALUES))
}
