#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate work_queue;
extern crate num_cpus;
extern crate rustc_hash;


mod functions;

mod board_types {
  pub mod normalboard;
  pub mod bitboard;
}

mod enums {
  pub mod chess_color;
  pub mod end_type;
  pub mod chess_error;
  pub mod piece_num;
  pub mod piece_type;
}

mod traits {
  pub mod chess_board_contract;
}

mod turn_functions {
  pub mod minimax_move;
  pub mod player_move;
}

mod evaluation_functions {
  pub mod board_piece_evaluation;
}

use crate::{
  functions::{
    parse_fen_to_normalboard,
    normalboard_to_fen
  },
  board_types::{
    normalboard::NormalBoard
  },
  enums::{
    chess_color::ChessColor,
    chess_error::ChessError
  }
};

use crate::turn_functions::{
  player_move::player_move
};

use board_types::bitboard::{Constants, BitBoard};
use turn_functions::minimax_move::minimax_move;

use traits::{
  chess_board_contract::ChessBoardContract
};

pub type EvaluationFunction<T: ChessBoardContract> = fn(&T, &Vec<T>, i32, &Constants) -> Result<i32, ChessError>;

pub struct Player<T: 'static + ChessBoardContract> {
    turn_function: Box<dyn Fn(&T, &Vec<T>, ChessColor, &Player<T>, &Constants) -> Result<String, ChessError>>,
    moves_ahead: i32
}

impl<T: 'static + ChessBoardContract + Clone + Send + Sync> Player<T> {
    pub fn human_player() -> Self {
        Self {
            turn_function: Box::new(player_move),
            moves_ahead: 0
        }
    }

    pub fn minimax_bot(moves_ahead: i32, eval_func: EvaluationFunction<T>, alpha_beta_pruning: bool, multi_threading: bool) -> Self {
        Self {
            turn_function: {
                Box::new(move |board: &T, board_history: &Vec<T>, turn: ChessColor, player: &Player<T>, constants: &Constants| -> Result<String, ChessError> {
                    minimax_move(board, board_history, turn, player, eval_func, constants, alpha_beta_pruning, multi_threading)
                })
            },
            moves_ahead: moves_ahead
        }
    }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![fen_to_possible_moves])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn fen_to_possible_moves(fen: String) -> Vec<(String, String)> {
  println!("{}", fen);
  let (normalboard, turn) = parse_fen_to_normalboard(&fen);

  println!("{}", normalboard_to_fen(&normalboard, turn));
  println!("{}", normalboard.board_ascii(true));

  normalboard.generate_possible_moves(turn)
  .expect("Test!")
  .into_iter()
  .map(|(mov, board)| (mov, normalboard_to_fen(&board, turn.opposite_color()))).collect()
}
