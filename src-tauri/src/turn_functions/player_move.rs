// use crate::board_types::bitboard::Constants;
// use crate::enums::{
//     chess_color::ChessColor,
//     chess_error::ChessError
// };

// use crate::functions::{
//     validate_move_string
// };

// use crate::Player;
// use crate::traits::chess_board_contract::ChessBoardContract;

// use std::io;
// use std::io::Write;

// pub fn player_move<T: ChessBoardContract>(board: &T, board_history: &Vec<T>, turn: ChessColor, player: &Player<T>, constants: &Constants) -> Result<String, ChessError> {
//     let color_str = match turn {
//         ChessColor::White => "White",
//         ChessColor::Black => "Black"
//     };

//     println!("It is {}'s turn! (You)\n", color_str);
//     println!("{}", board.board_ascii(true));

//     let move_str = loop {
//         print!("\nEnter your move: ");
//         std::io::stdout().flush();
    
//         let mut inp = String::new();
//         io::stdin().read_line(&mut inp);

//         if inp.len() < 5 {
//             print!("Invalid string!");
//             continue;
//         }
//         inp = inp[0..5].to_string();

//         let valid_move = validate_move_string(&inp);
//         if !valid_move {
//             print!("Invalid string!");
//             continue;
//         }

//         let valid_move = board.generate_moves(turn, constants)?.iter().any(|mov| (*mov).0 == inp);
//         if !valid_move {
//             print!("Invalid move!");
//             continue;
//         }

//         break inp;
//     };
    
//     Ok(move_str)
// }
