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
  pub mod piece_color;
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
    fen_to_normalboard
  },
  board_types::{
    normalboard::NormalBoard
  },
  enums::{
    piece_color::PieceColor,
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

pub type EvaluationFunction<T: ChessBoardContract> = fn(&T, Option<&T>, &Vec<T>, i32, &Constants) -> Result<i32, ChessError>;

pub struct Player<T: 'static + ChessBoardContract> {
    turn_function: Box<dyn Fn(&T, Option<&T>, &Vec<T>, PieceColor, &Player<T>, &Constants) -> Result<String, ChessError>>,
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
                Box::new(move |board: &T, previous_board: Option<&T>, board_history: &Vec<T>, turn: PieceColor, player: &Player<T>, constants: &Constants| -> Result<String, ChessError> {
                    minimax_move(board, previous_board, board_history, turn, player, eval_func, constants, alpha_beta_pruning, multi_threading)
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
fn fen_to_possible_moves(fen: String) -> Vec<String> {
  println!("{}", fen);

  let mut split = fen.split_whitespace();

  let board_string = split.nth(0).expect("FEN board part");
  let turn_string = split.nth(0).expect("FEN turn part");
  let castle_sttring = split.nth(0).expect("FEN castle part");
  let passent_string = split.nth(0).expect("FEN en passent part");
  let half_move_string = split.nth(0).expect("FEN half move part");
  let full_move_string = split.nth(0).expect("FEN full move part");

  println!("{}", board_string);
  println!("{}", turn_string);
  println!("{}", castle_sttring);
  println!("{}", passent_string);
  println!("{}", half_move_string);
  println!("{}", full_move_string);

  let normalboard: NormalBoard = fen_to_normalboard(board_string);
  let turn = match turn_string {
      "w" => PieceColor::White,
      _ => PieceColor::Black
  };

  normalboard.generate_possible_moves(None, turn)
  .expect("Test!")
  .into_iter()
  .map(|x| x.0).collect()
}
