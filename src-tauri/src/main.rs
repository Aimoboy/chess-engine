#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate work_queue;
extern crate num_cpus;
extern crate rustc_hash;
extern crate sha1;
extern crate xxhash_rust;


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

use std::{vec, hash::{Hash, Hasher}, collections::HashMap};

use crate::{
  functions::{
    parse_fen_to_normalboard
  },
  board_types::{
    normalboard::NormalBoard
  },
  enums::{
    chess_color::ChessColor,
    chess_error::ChessError
  }
};

// use crate::turn_functions::{
//   player_move::player_move
// };

use board_types::bitboard::{Constants, BitBoard};
use enums::end_type::EndType;
use crate::evaluation_functions::board_piece_evaluation::board_piece_evaluation;
use serde::{Serialize, Deserialize};
use turn_functions::minimax_move::minimax_move;

use sha1::{Sha1, Digest};
use xxhash_rust::xxh3::xxh3_64;

use traits::{
  chess_board_contract::ChessBoardContract
};

pub type EvaluationFunction<T> = fn(&T, &HashMap<T, i32>, ChessColor, i32, &Constants) -> Result<i32, ChessError>;

// pub struct Player<T: 'static + ChessBoardContract> {
//     turn_function: Box<dyn Fn(&T, &Vec<T>, ChessColor, &Player<T>, &Constants) -> Result<String, ChessError>>,
//     moves_ahead: i32
// }

// impl<T: 'static + ChessBoardContract + Clone + Send + Sync> Player<T> {
//     pub fn human_player() -> Self {
//         Self {
//             turn_function: Box::new(player_move),
//             moves_ahead: 0
//         }
//     }

//     pub fn minimax_bot(moves_ahead: i32, eval_func: EvaluationFunction<T>, alpha_beta_pruning: bool, multi_threading: bool) -> Self {
//         Self {
//             turn_function: {
//                 Box::new(move |board: &T, board_history: &HashMap<T, i32>, turn: ChessColor, player: &Player<T>, constants: &Constants| -> Result<String, ChessError> {
//                     minimax_move(board, board_history, turn, player, eval_func, constants, alpha_beta_pruning, multi_threading)
//                 })
//             },
//             moves_ahead: moves_ahead
//         }
//     }
// }

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![handle_request, get_start_board_state])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Deserialize, Debug)]
enum PlayerType {
  Human = 0,
  Minimax = 1
}

#[derive(Deserialize)]
struct PlayerConfig {
  player_type: i32,
  alpha_beta_pruning: bool,
  multi_threading: bool,
  moves_ahead: i32
}

#[tauri::command(async)]
fn handle_request(board_history: Vec<String>, white_player_config: PlayerConfig, black_player_config: PlayerConfig) -> Result<BoardState, ChessError> {
  // let (current_fen, board_history_hashmap) = fen_history_to_current_fen_and_history_hashmap(&board_history);
  let state = generate_board_state_from_fen(&board_history)?;

  if state.turn == ChessColor::White && white_player_config.player_type == PlayerType::Human as i32 ||
     state.turn == ChessColor::Black && black_player_config.player_type == PlayerType::Human as i32 {
    Ok(state)
  } else {
    let last = state.history.last().unwrap();
    let (normalboard, _) = parse_fen_to_normalboard(last);
    let board_history_hashmap = generate_board_history_hashmap_from_fens(&state.history);

    let (alpha_beta_pruning, multi_threading, moves_ahead) = match state.turn {
        ChessColor::White => (white_player_config.alpha_beta_pruning, white_player_config.multi_threading, white_player_config.moves_ahead),
        ChessColor::Black => (black_player_config.alpha_beta_pruning, black_player_config.multi_threading, black_player_config.moves_ahead)
    };

    let mov = minimax_move(&normalboard, &board_history_hashmap, state.turn, moves_ahead, board_piece_evaluation, &Constants::empty(), alpha_beta_pruning, multi_threading).unwrap();
    let mov_fen = &state.moves.iter().filter(|(m, fen)| {
      mov == *m
    }).into_iter().collect::<Vec<_>>().first().unwrap().1;

    let mut new_history = state.history.clone();
    new_history.push(mov_fen.clone());

    generate_board_state_from_fen(&new_history)
  }
}

// fn fen_history_to_current_fen_and_history_hashmap(board_history: &[String]) -> (String, HashMap<NormalBoard, i32>) {
//   let current = board_history.last().unwrap();
//   let board_history_hashmap = generate_board_history_hashmap_from_fens(board_history);

//   (current.clone(), board_history_hashmap)
// }















// #[tauri::command]
// fn fen_to_board_state(history: Vec<String>) -> Result<BoardState, ChessError> {
//   let state = generate_board_state_from_fen(&history);

//   if let Ok(state) = &state {
//     if state.turn == ChessColor::Black {
//       let last = state.history.last().unwrap();
//       let (normalboard, _) = parse_fen_to_normalboard(last);
//       let board_history = generate_board_history_hashmap_from_fens(&state.history);

//       let mov = minimax_move(&normalboard, &board_history, ChessColor::Black, 3, board_piece_evaluation, &Constants::empty(), true, true).unwrap();
//       let mov_fen = &state.moves.iter().filter(|(m, fen)| {
//         mov == *m
//       }).into_iter().collect::<Vec<_>>().first().unwrap().1;

//       let mut new_history = state.history.clone();
//       new_history.push(mov_fen.clone());

//       return generate_board_state_from_fen(&new_history);
//     }
//   }

//   state
// }

#[tauri::command]
fn get_start_board_state() -> Result<BoardState, ChessError> {
  BoardState::get_start()
}

#[derive(Serialize)]
struct BoardState {
  fen: String,
  win_state: EndType,
  turn: ChessColor,
  moves: Vec<(String, String)>,
  history: Vec<String>
}

impl BoardState {
  pub fn get_start() -> Result<BoardState, ChessError> {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    generate_board_state_from_fen(vec![fen.to_owned()].as_ref())
  }
}

fn generate_board_state_from_fen(history: &Vec<String>) -> Result<BoardState, ChessError> {
  let fen = history.last().unwrap();
  let (normalboard, turn) = parse_fen_to_normalboard(fen);

  let board_history_hashmap = generate_board_history_hashmap_from_fens(&history);
  let win_state = normalboard.check_for_game_end(turn, &board_history_hashmap)?;

  let moves = match win_state == EndType::NoEnd {
      true => {
        normalboard.generate_possible_moves(turn)?
        .into_iter()
        .map(|(mov, board)| {
          (mov, board.board_to_fen(turn.opposite_color()))
        }).collect()
      },
      false => {
        Vec::new()
      }
  };
   

  Ok(BoardState {
    fen: fen.to_owned(),
    turn,
    win_state,
    moves,
    history: history.clone()
  })
}

fn generate_board_history_hashmap_from_fens(history: &[String]) -> HashMap<NormalBoard, i32> {
  let mut hashmap = HashMap::new();

  history.iter().for_each(|fen| {
    let (board, _) = parse_fen_to_normalboard(fen);
    match hashmap.get_mut(&board) {
      Some(val) => {
        *val += 1;
      },
      None => {
        hashmap.insert(board, 1);
      }
    }
  });

  hashmap
}
