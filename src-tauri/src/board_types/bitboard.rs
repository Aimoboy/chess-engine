use crate::enums::chess_error::ChessError;
use crate::functions::{
    get_letter,
    get_number,
    pos_to_num,
    num_to_pos
};

use crate::traits::{
    chess_board_contract::ChessBoardContract
};

use crate::board_types::normalboard::{
    ChessPiece,
    NormalBoard
};

use crate::enums::{
    chess_color::ChessColor,
    end_type::EndType,
    piece_num::PieceNum,
    piece_type::PieceType
};

use rustc_hash::FxHashMap;

pub type BitBoard = [u64; 12];
pub type BitBoardMove = (((u64, u64), (u64, u64)), BitBoard);

#[derive(Clone)]
pub struct Constants {
    pub start_board: BitBoard,
    pub row_and_column_mask: [u64; 64],
    pub diagonal_mask: [u64; 64],
    pub pawn_reach: [[u64; 64]; 2],
    pub rook_reach: [FxHashMap<u64, u64>; 64],
    pub knight_reach: [u64; 64],
    pub bishop_reach: [FxHashMap<u64, u64>; 64],
    pub king_reach: [u64; 64]
}

impl Constants {
    pub fn new() -> Self {
        Self {
            start_board: generate_start_board(),
            row_and_column_mask: generate_row_and_column_mask(),
            diagonal_mask: generate_diagonal_mask(),
            pawn_reach: generate_pawn_reach(),
            rook_reach: generate_rook_reach(),
            knight_reach: generate_knight_reach(),
            bishop_reach: generate_bishop_reach(),
            king_reach: generate_king_reach()
        }
    }

    pub fn empty() -> Self {
        Self {
            start_board: [0; 12],
            row_and_column_mask: [0; 64],
            diagonal_mask: [0; 64],
            pawn_reach: [[0; 64]; 2],
            rook_reach: [0; 64].map(|_| FxHashMap::default()),
            knight_reach: [0; 64],
            bishop_reach: [0; 64].map(|_| FxHashMap::default()),
            king_reach: [0; 64]
        }
    }
}

pub fn generate_all_possible_configurations(input_board: u64) -> Vec<u64> {
    let mut configurations: Vec<u64> = Vec::with_capacity(100_000);
    let mut points: Vec<u64> = Vec::with_capacity(100);

    let mut tmp = input_board;
    while tmp > 0 {
        let point = tmp.trailing_zeros() as u64;
        points.push(point);
        tmp -= 1 << point;
    }

    configurations.push(0);

    let mut board = 0;
    let mut count = 0;
    while count < points.len() {
        let num_pos = points[count];
        if board & (1 << num_pos) == 0 {
            board += 1 << num_pos;
            for l in 0..count {
                let num_pos = points[l];
                board -= 1 << num_pos;
            }
            configurations.push(board);
            count = 0;
        } else {
            count += 1;
        }
    }

    configurations
}

pub fn generate_start_board() -> BitBoard {
    let mut board = [0; 12];

    // White pawns
    for i in 0..8 {
        board[PieceNum::WhitePawn as usize] += 1 << pos_to_num(i, 1);
    }

    // White rooks
    board[PieceNum::WhiteRook as usize] += 1 << pos_to_num(0, 0);
    board[PieceNum::WhiteRook as usize] += 1 << pos_to_num(7, 0);

    // White knights
    board[PieceNum::WhiteKnight as usize] += 1 << pos_to_num(1, 0);
    board[PieceNum::WhiteKnight as usize] += 1 << pos_to_num(6, 0);

    // White bishops
    board[PieceNum::WhiteBishop as usize] += 1 << pos_to_num(2, 0);
    board[PieceNum::WhiteBishop as usize] += 1 << pos_to_num(5, 0);

    // White queen
    board[PieceNum::WhiteQueen as usize] += 1 << pos_to_num(3, 0);

    // White king
    board[PieceNum::WhiteKing as usize] += 1 << pos_to_num(4, 0);

    // Black pawns
    for i in 0..8 {
        board[PieceNum::BlackPawn as usize] += 1 << pos_to_num(i, 6);
    }

    // Black rooks
    board[PieceNum::BlackRook as usize] += 1 << pos_to_num(0, 7);
    board[PieceNum::BlackRook as usize] += 1 << pos_to_num(7, 7);

    // Black knights
    board[PieceNum::BlackKnight as usize] += 1 << pos_to_num(1, 7);
    board[PieceNum::BlackKnight as usize] += 1 << pos_to_num(6, 7);

    // Black bishops
    board[PieceNum::BlackBishop as usize] += 1 << pos_to_num(2, 7);
    board[PieceNum::BlackBishop as usize] += 1 << pos_to_num(5, 7);

    // Black queen
    board[PieceNum::BlackQueen as usize] += 1 << pos_to_num(3, 7);

    // Black king
    board[PieceNum::BlackKing as usize] += 1 << pos_to_num(4, 7);

    board
}

pub fn generate_row_and_column_mask() -> [u64; 64] {
    let mut possible_moves = [0; 64];

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut tmp = 0;

        // Up
        for j in number + 1..8 {
            tmp += 1 << pos_to_num(letter, j);
        }

        // Down
        for j in 0..number {
            tmp += 1 << pos_to_num(letter, j);
        }

        // Left
        for j in 0..letter {
            tmp += 1 << pos_to_num(j, number);
        }

        // Right
        for j in letter + 1..8 {
            tmp += 1 << pos_to_num(j, number);
        }

        possible_moves[i as usize] = tmp;
    }


    possible_moves
}

// pub fn generate_double_row_and_column_mask(row_column_mask: [u64; 64]) ->  HashMap<u64, u64> { //HashMap<u64, u64> {
//     let mut mask_map: HashMap<u64, u64> = HashMap::with_capacity(2081);

//     let mut possibilities = [0; 2081];

//     // Index starts from 1 because you can have zero pieces of the type on the board
//     for i in 0..64 {
//         possibilities[i + 1] = 1 << i;
//     }

//     let mut index = 65;
//     for i in 1..64 {
//         for j in 0..i {
//             possibilities[index] = (1 << i) + (1 << j);
//             index += 1;
//         }
//     }

//     mask_map.insert(0, 0);

//     for possibility in &possibilities[1 .. 64] {
//         mask_map.insert(*possibility, row_column_mask[possibility.trailing_zeros() as usize]);
//     }

//     for possibility in &possibilities[65 .. ] {
//         let first_pos = (*possibility as u64).trailing_zeros() as usize;
//         let second_pos = ((*possibility - (1 << first_pos)) as u64).trailing_zeros() as usize;

//         mask_map.insert(*possibility, row_column_mask[first_pos] | row_column_mask[second_pos]);
//     }


//     // let mut test = std::collections::HashSet::with_capacity(300_000_000);
//     // // 268_435_456
//     // // let mut test = 0;
//     // for mov in &mask_map {
//     //     let (_, mask) = match mov {
//     //         (a, b) => (*a, *b)
//     //     };

//     //     let configurations = generate_all_possible_configurations(mask);
        
//     //     // test += configurations.len() as u64;
//     //     for item in configurations {
//     //         test.insert(item);
//     //     }
//     // }

//     // println!("{}", test.len());

//     mask_map
// }

fn generate_diagonal_mask() -> [u64; 64] {
    let mut possible_moves = [0; 64];

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut tmp = 0;

        // Up-right
        for j in 1..7 {
            let new_letter = letter as i32 + j;
            let new_number = number as i32 + j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            tmp += 1 << pos_to_num(new_letter as u64, new_number as u64);
        }

        // Down-right
        for j in 1..7 {
            let new_letter = letter as i32 + j;
            let new_number = number as i32 - j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            tmp += 1 << pos_to_num(new_letter as u64, new_number as u64);
        }

        // Down-left
        for j in 1..7 {
            let new_letter = letter as i32 - j;
            let new_number = number as i32 - j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            tmp += 1 << pos_to_num(new_letter as u64, new_number as u64);
        }

        // Up-left
        for j in 1..7 {
            let new_letter = letter as i32 - j;
            let new_number = number as i32 + j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            tmp += 1 << pos_to_num(new_letter as u64, new_number as u64);
        }

        possible_moves[i as usize] = tmp;
    }


    possible_moves
}

// First = White, Second = Black
fn generate_pawn_reach() -> [[u64; 64]; 2] {
    let mut possible_moves = [[0; 64]; 2];
    let side_constants = [1, -1];

    for side in side_constants {
        for i in 0..64 {
            let (letter, number) = num_to_pos(i);
            let mut b = 0;

            // Right
            let new_letter = letter as i32 + 1;
            let new_number = number as i32 + side;
            if 0 <= new_letter && new_letter < 8 && 0 <= new_number && new_number < 8 {
                b += 1 << pos_to_num(new_letter as u64, new_number as u64);
            }

            // Left
            let new_letter = letter as i32 - 1;
            let new_number = number as i32 + side;
            if 0 <= new_letter && new_letter < 8 && 0 <= new_number && new_number < 8 {
                b += 1 << pos_to_num(new_letter as u64, new_number as u64);
            }

            if side == 1 {
                possible_moves[0][i as usize] = b;
            } else {
                possible_moves[1][i as usize] = b;
            }
        }
    }

    possible_moves
}

fn generate_rook_reach() -> [FxHashMap<u64, u64>; 64] {
    let mut possible_moves: [FxHashMap<u64, u64>; 64] = [(); 64].map(|_| FxHashMap::default());

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut possibilities = Vec::with_capacity(16384);
        possibilities.push(0);
        let mut points = Vec::with_capacity(14);

        // Up
        for j in number + 1..8 {
            points.push(pos_to_num(letter, j));
        }

        // Down
        for j in 0..number {
            points.push(pos_to_num(letter, j));
        }

        // Left
        for j in 0..letter {
            points.push(pos_to_num(j, number));
        }

        // Right
        for j in letter + 1..8 {
            points.push(pos_to_num(j, number));
        }

        let mut board = 0;
        let mut count = 0;
        while count < points.len() {
            let num_pos = points[count];
            if board & (1 << num_pos) == 0 {
                board += 1 << num_pos;
                for l in 0..count {
                    let num_pos = points[l];
                    board -= 1 << num_pos;
                }
                possibilities.push(board);
                count = 0;
            } else {
                count += 1;
            }
        }

        for p in possibilities {
            let mut board = 0;

            // Up
            for j in number + 1..8 {
                let pos = 1 << pos_to_num(letter, j);
                board += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Down
            for j in (0..number).rev() {
                let pos = 1 << pos_to_num(letter, j);
                board += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Left
            for j in (0..letter).rev() {
                let pos = 1 << pos_to_num(j, number);
                board += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Right
            for j in letter + 1..8 {
                let pos = 1 << pos_to_num(j, number);
                board += pos;
                if p & pos == pos {
                    break;
                }
            }

            possible_moves[i as usize].insert(p, board);
        }
    }

    possible_moves
}

fn generate_knight_reach() -> [u64; 64] {
    let mut possible_moves = [0; 64];

    // Clockwise starting from up-right
    let position_diffs = [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)];

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut board = 0;

        for diffs in position_diffs {
            let (letter_diff, number_diff) = diffs;
            let new_letter = letter as i32 + letter_diff;
            let new_number = number as i32 + number_diff;
            if 0 <= new_letter && new_letter < 8 && 0 <= new_number && new_number < 8 {
                board += 1 << pos_to_num(new_letter as u64, new_number as u64);
            }
        }

        possible_moves[i as usize] = board;
    }

    possible_moves
}

fn generate_bishop_reach() -> [FxHashMap<u64, u64>; 64] {
    let mut possible_moves: [FxHashMap<u64, u64>; 64] = [(); 64].map(|_| FxHashMap::default());

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut possibilities = Vec::with_capacity(16384);
        possibilities.push(0);
        let mut points = Vec::with_capacity(14);

        // Up-right
        for j in 1..7 {
            let new_letter = letter as i32 + j;
            let new_number = number as i32 + j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            points.push(pos_to_num(new_letter as u64, new_number as u64));
        }

        // Down-right
        for j in 1..7 {
            let new_letter = letter as i32 + j;
            let new_number = number as i32 - j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            points.push(pos_to_num(new_letter as u64, new_number as u64));
        }

        // Down-left
        for j in 1..7 {
            let new_letter = letter as i32 - j;
            let new_number = number as i32 - j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            points.push(pos_to_num(new_letter as u64, new_number as u64));
        }

        // Up-left
        for j in 1..7 {
            let new_letter = letter as i32 - j;
            let new_number = number as i32 + j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            points.push(pos_to_num(new_letter as u64, new_number as u64));
        }

        let mut board = 0;
        let mut count = 0;
        while count < points.len() {
            let num_pos = points[count];
            if board & (1 << num_pos) == 0 {
                board += 1 << num_pos;
                for l in 0..count {
                    let num_pos = points[l];
                    board -= 1 << num_pos;
                }
                possibilities.push(board);
                count = 0;
            } else {
                count += 1;
            }
        }

        for p in possibilities {
            let mut b = 0;

            // Up-right
            for j in 1..7 {
                let new_letter = letter as i32 + j;
                let new_number = number as i32 + j;
                if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                    break;
                }
                let pos = 1 << pos_to_num(new_letter as u64, new_number as u64);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Down-right
            for j in 1..7 {
                let new_letter = letter as i32 + j;
                let new_number = number as i32 - j;
                if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                    break;
                }
                let pos = 1 << pos_to_num(new_letter as u64, new_number as u64);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Down-left
            for j in 1..7 {
                let new_letter = letter as i32 - j;
                let new_number = number as i32 - j;
                if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                    break;
                }
                let pos = 1 << pos_to_num(new_letter as u64, new_number as u64);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Up-left
            for j in 1..7 {
                let new_letter = letter as i32 - j;
                let new_number = number as i32 + j;
                if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                    break;
                }
                let pos = 1 << pos_to_num(new_letter as u64, new_number as u64);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            possible_moves[i as usize].insert(p, b);
        }
    }

    possible_moves
}

fn generate_king_reach() -> [u64; 64] {
    let mut possible_moves = [0; 64];

    // Clockwise starting from up-right
    let position_diffs = [(1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1)];

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut board = 0;

        for diffs in position_diffs {
            let (letter_diff, number_diff) = diffs;
            let new_letter = letter as i32 + letter_diff;
            let new_number = number as i32 + number_diff;
            if 0 <= new_letter && new_letter < 8 && 0 <= new_number && new_number < 8 {
                board += 1 << pos_to_num(new_letter as u64, new_number as u64);
            }
        }

        possible_moves[i as usize] = board;
    }

    possible_moves
}

pub fn print_bitboard(num: u64) {
    let board = format!("{:#066b}", num).chars().rev().collect::<String>();
    for i in (0..8).rev() {
        println!("{}", board[i*8..i*8+8].to_string());
    }
}

fn get_full_color_board(board: &BitBoard, color: ChessColor) -> u64 {
    let mut possible_moves: u64 = 0;
    match color {
        ChessColor::White => {
            possible_moves |= board[PieceNum::WhitePawn as usize];
            possible_moves |= board[PieceNum::WhiteRook as usize];
            possible_moves |= board[PieceNum::WhiteKnight as usize];
            possible_moves |= board[PieceNum::WhiteBishop as usize];
            possible_moves |= board[PieceNum::WhiteQueen as usize];
            possible_moves |= board[PieceNum::WhiteKing as usize];
        },
        ChessColor::Black => {
            possible_moves |= board[PieceNum::BlackPawn as usize];
            possible_moves |= board[PieceNum::BlackRook as usize];
            possible_moves |= board[PieceNum::BlackKnight as usize];
            possible_moves |= board[PieceNum::BlackBishop as usize];
            possible_moves |= board[PieceNum::BlackQueen as usize];
            possible_moves |= board[PieceNum::BlackKing as usize];
        }
    }
    possible_moves
}

fn get_occupied_board(board: &BitBoard) -> u64 {
    get_full_color_board(board, ChessColor::White) | get_full_color_board(board, ChessColor::Black)
}

pub fn get_reach_board(board: &BitBoard, color: ChessColor, constants: &Constants) -> u64 {
    let occupied_board = get_occupied_board(board);

    let mut b = 0;

    match color {
        ChessColor::White => {
            for i in 0..64 {
                let num = 1 << i;
                let rows_and_columns = occupied_board & constants.row_and_column_mask[i];
                let diagonals = occupied_board & constants.diagonal_mask[i];

                if board[PieceNum::WhitePawn as usize] & num == num {
                    b |= constants.pawn_reach[0][i];
                } else if board[PieceNum::WhiteRook as usize] & num == num {
                    b |= constants.rook_reach[i].get(&rows_and_columns).unwrap();
                } else if board[PieceNum::WhiteKnight as usize] & num == num {
                    b |= constants.knight_reach[i];
                } else if board[PieceNum::WhiteBishop as usize] & num == num {
                    b |= constants.bishop_reach[i].get(&diagonals).unwrap();
                } else if board[PieceNum::WhiteQueen as usize] & num == num {
                    b |= constants.rook_reach[i].get(&rows_and_columns).unwrap();
                    b |= constants.bishop_reach[i].get(&diagonals).unwrap();
                } else if board[PieceNum::WhiteKing as usize] & num == num {
                    b |= constants.king_reach[i];
                }
            }
        },
        ChessColor::Black => {
            for i in 0..64 {
                let num = 1 << i;
                let rows_and_columns = occupied_board & constants.row_and_column_mask[i];
                let diagonals = occupied_board & constants.diagonal_mask[i];

                if board[PieceNum::BlackPawn as usize] & num == num {
                    b |= constants.pawn_reach[1][i];
                } else if board[PieceNum::BlackRook as usize] & num == num {
                    b |= constants.rook_reach[i].get(&rows_and_columns).unwrap();
                } else if board[PieceNum::BlackKnight as usize] & num == num {
                    b |= constants.knight_reach[i];
                } else if board[PieceNum::BlackBishop as usize] & num == num {
                    b |= constants.bishop_reach[i].get(&diagonals).unwrap();
                } else if board[PieceNum::BlackQueen as usize] & num == num {
                    b |= constants.rook_reach[i].get(&rows_and_columns).unwrap();
                    b |= constants.bishop_reach[i].get(&diagonals).unwrap();
                } else if board[PieceNum::BlackKing as usize] & num == num {
                    b |= constants.king_reach[i];
                }
            }
        }
    }

    b
}

fn is_in_check(board: &BitBoard, color: ChessColor, constants: &Constants) -> bool {
    let opposite_reach_board = get_reach_board(&board, color.opposite_color(), constants);
    match color {
        ChessColor::White => {
            let king_num = board[PieceNum::WhiteKing as usize];
            opposite_reach_board & king_num == king_num
        },
        ChessColor::Black => {
            let king_num = board[PieceNum::BlackKing as usize];
            opposite_reach_board & king_num == king_num
        }
    }
}

pub fn generate_possible_moves(board: &BitBoard, color: ChessColor, constants: &Constants) -> Vec<BitBoardMove> {
    let mut possible_moves = Vec::new();

    let opposite_color = color.opposite_color();
    let occupied_board = get_occupied_board(&board);
    let own_pieces = get_full_color_board(&board, color);
    let opposite_pieces = get_full_color_board(&board, opposite_color);
    let reach_board = get_reach_board(&board, color, constants);
    let opposite_reach_board = get_reach_board(&board, opposite_color, constants);

    match color {
        ChessColor::White => {
            // Pawns
            let mut tmp = board[PieceNum::WhitePawn as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                if occupied_board & 1 << (i + 8) == 0 {
                    let mut new_board = board.clone();
                    new_board[PieceNum::WhitePawn as usize] -= 1 << i;
                    if 55 <= i + 8 && i + 8 < 64 {
                        new_board[PieceNum::WhiteQueen as usize] += 1 << (i + 8);
                    } else {
                        new_board[PieceNum::WhitePawn as usize] += 1 << (i + 8);
                    }
                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 + 8)), new_board));

                    if 7 <= i && i < 16 && occupied_board & 1 << (i + 16) == 0 {
                        let mut new_board = board.clone();
                        new_board[PieceNum::WhitePawn as usize] -= 1 << i;
                        new_board[PieceNum::WhitePawn as usize] += 1 << (i + 16);
                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 + 16)), new_board));
                    }
                }

                let mut possible_attacks = constants.pawn_reach[0][i as usize] & opposite_pieces;
                while possible_attacks != 0 {
                    let j = possible_attacks.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhitePawn as usize] -= 1 << i;
                    new_board[PieceNum::WhitePawn as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    possible_attacks -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Rooks
            let mut tmp = board[PieceNum::WhiteRook as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteRook as usize] -= 1 << i;
                    new_board[PieceNum::WhiteRook as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Knights
            let mut tmp = board[PieceNum::WhiteKnight as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.knight_reach[i as usize];
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteKnight as usize] -= 1 << i;
                    new_board[PieceNum::WhiteKnight as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Bishops
            let mut tmp = board[PieceNum::WhiteBishop as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteBishop as usize] -= 1 << i;
                    new_board[PieceNum::WhiteBishop as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Queen
            let mut tmp = board[PieceNum::WhiteQueen as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable_straight = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                let reachable_diagonals = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                let reachable = reachable_straight | reachable_diagonals;
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteQueen as usize] -= 1 << i;
                    new_board[PieceNum::WhiteQueen as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // King
            let mut tmp = board[PieceNum::WhiteKing as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.king_reach[i as usize];
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteKing as usize] -= 1 << i;
                    new_board[PieceNum::WhiteKing as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }
        },
        ChessColor::Black => {
            // Pawns
            let mut tmp = board[PieceNum::BlackPawn as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                if occupied_board & 1 << (i - 8) == 0 {
                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackPawn as usize] -= 1 << i;
                    // if 0 <= i - 8 && i - 8 < 8 {
                    if i - 8 < 8 {
                        new_board[PieceNum::BlackQueen as usize] += 1 << (i - 8);
                    } else {
                        new_board[PieceNum::BlackPawn as usize] += 1 << (i - 8);
                    }
                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 - 8)), new_board));

                    // if 48 <= i && i < 56 && occupied_board & 1 << (i - 16) == 0 {
                    if (48..56).contains(&i) && occupied_board & 1 << (i - 16) == 0 {
                        let mut new_board = board.clone();
                        new_board[PieceNum::BlackPawn as usize] -= 1 << i;
                        new_board[PieceNum::BlackPawn as usize] += 1 << (i - 16);
                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 - 16)), new_board));
                    }
                }

                let mut possible_attacks = constants.pawn_reach[1][i as usize] & opposite_pieces;
                while possible_attacks != 0 {
                    let j = possible_attacks.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackPawn as usize] -= 1 << i;
                    new_board[PieceNum::BlackPawn as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    possible_attacks -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Rooks
            let mut tmp = board[PieceNum::BlackRook as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackRook as usize] -= 1 << i;
                    new_board[PieceNum::BlackRook as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Knights
            let mut tmp = board[PieceNum::BlackKnight as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.knight_reach[i as usize];
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackKnight as usize] -= 1 << i;
                    new_board[PieceNum::BlackKnight as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Bishops
            let mut tmp = board[PieceNum::BlackBishop as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackBishop as usize] -= 1 << i;
                    new_board[PieceNum::BlackBishop as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Queen
            let mut tmp = board[PieceNum::BlackQueen as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable_straight = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                let reachable_diagonals = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                let reachable = reachable_straight | reachable_diagonals;
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackQueen as usize] -= 1 << i;
                    new_board[PieceNum::BlackQueen as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // King
            let mut tmp = board[PieceNum::BlackKing as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.king_reach[i as usize];
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackKing as usize] -= 1 << i;
                    new_board[PieceNum::BlackKing as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }
        }
    }

    // Validate
    let mut res = Vec::new();
    for mov in possible_moves {
        let b = mov.1;

        if !is_in_check(&b, color, constants) {
            res.push(mov);
        }
    }

    res
}

fn piece_to_char(piece: &ChessPiece) -> char {
    match piece.color {
        ChessColor::White => match piece.typ {
                                PieceType::Pawn => 'p',
                                PieceType::Rook => 'r',
                                PieceType::Knight => 'n',
                                PieceType::Bishop => 'b',
                                PieceType::Queen => 'q',
                                PieceType::King => 'k'
                            }
        ChessColor::Black => match piece.typ {
                                PieceType::Pawn => 'P',
                                PieceType::Rook => 'R',
                                PieceType::Knight => 'N',
                                PieceType::Bishop => 'B',
                                PieceType::Queen => 'Q',
                                PieceType::King => 'K'
                            }
    }

    
}

fn get_piece_str(board: &BitBoard, letter: u64, number: u64) -> &str {
    let num = 1 << pos_to_num(letter, number);

    if board[PieceNum::WhitePawn as usize] & num == num {
        "p"
    } else if board[PieceNum::WhiteRook as usize] & num == num {
        "r"
    } else if board[PieceNum::WhiteKnight as usize] & num == num {
        "n"
    } else if board[PieceNum::WhiteBishop as usize] & num == num {
        "b"
    } else if board[PieceNum::WhiteQueen as usize] & num == num {
        "q"
    } else if board[PieceNum::WhiteKing as usize] & num == num {
        "k"
    } else if board[PieceNum::BlackPawn as usize] & num == num {
        "P"
    } else if board[PieceNum::BlackRook as usize] & num == num {
        "R"
    } else if board[PieceNum::BlackKnight as usize] & num == num {
        "N"
    } else if board[PieceNum::BlackBishop as usize] & num == num {
        "B"
    } else if board[PieceNum::BlackQueen as usize] & num == num {
        "Q"
    } else if board[PieceNum::BlackKing as usize] & num == num {
        "K"
    } else {
        " "
    }
}

fn get_piece_unicode(board: &BitBoard, letter: u64, number: u64) -> &str{
    let num = 1 << pos_to_num(letter, number);

    if board[PieceNum::WhitePawn as usize] & num == num {
        "\u{265F}"
    } else if board[PieceNum::WhiteRook as usize] & num == num {
        "\u{265C} "
    } else if board[PieceNum::WhiteKnight as usize] & num == num {
        "\u{265E} "
    } else if board[PieceNum::WhiteBishop as usize] & num == num {
        "\u{265D} "
    } else if board[PieceNum::WhiteQueen as usize] & num == num {
        "\u{265B} "
    } else if board[PieceNum::WhiteKing as usize] & num == num {
        "\u{265A} "
    } else if board[PieceNum::BlackPawn as usize] & num == num {
        "\u{2659} "
    } else if board[PieceNum::BlackRook as usize] & num == num {
        "\u{2656} "
    } else if board[PieceNum::BlackKnight as usize] & num == num {
        "\u{2658} "
    } else if board[PieceNum::BlackBishop as usize] & num == num {
        "\u{2657} "
    } else if board[PieceNum::BlackQueen as usize] & num == num {
        "\u{2655} "
    } else if board[PieceNum::BlackKing as usize] & num == num {
        "\u{2654} "
    } else {
        "  "
    }
}

pub fn get_bitboard_ascii(board: &BitBoard, use_unicode: bool) -> String {
    let mut string = if use_unicode {
        String::with_capacity(844)
    } else {
        String::with_capacity(645)
    };
    
    for i in (0..8).rev() {
        if use_unicode {
            string.push_str("  +----+----+----+----+----+----+----+----+\n");
        } else {
            string.push_str("  +---+---+---+---+---+---+---+---+\n");
        }

        if let Some(res) = std::char::from_digit(1 + i as u32, 10) {
            string.push(res);
            string.push(' ');
        }
        for j in 0..8 {
            if use_unicode {
                string.push_str("| ");
                string.push_str(get_piece_unicode(&board, j as u64, i as u64));
                string.push(' ');
            } else {
                string.push_str("| ");
                string.push_str(get_piece_str(&board, j as u64, i as u64));
                string.push(' ');
            }
        }
        string.push_str("|\n");
    }

    if use_unicode {
        string.push_str("  +----+----+----+----+----+----+----+----+\n");
        string.push_str("    A    B    C    D    E    F    G    H");
    } else {
        string.push_str("  +---+---+---+---+---+---+---+---+\n");
        string.push_str("   A   B   C   D   E   F   G   H");
    }

    string
}

pub fn board_to_bitboard(board: &NormalBoard) -> BitBoard{
    let mut res = [0; 12];
    for i in 0..8 {
        for j in 0..8 {
            if let Ok(Some(piece)) = board.get_piece(i, j) {
                match piece.color {
                    ChessColor::White => {
                        match piece.typ {
                            PieceType::Pawn => {
                                res[0] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Rook => {
                                res[1] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Knight => {
                                res[2] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Bishop => {
                                res[3] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Queen => {
                                res[4] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::King => {
                                res[5] += 1 << pos_to_num(i as u64, j as u64);
                            }
                        }
                    },
                    ChessColor::Black => {
                        match piece.typ {
                            PieceType::Pawn => {
                                res[6] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Rook => {
                                res[7] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Knight => {
                                res[8] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Bishop => {
                                res[9] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::Queen => {
                                res[10] += 1 << pos_to_num(i as u64, j as u64);
                            },
                            PieceType::King => {
                                res[11] += 1 << pos_to_num(i as u64, j as u64);
                            }
                        }
                    }
                }
            }
        }
    }

    res
}

pub fn bitboard_check_game_end(bb: &BitBoard, turn: ChessColor, constants: &Constants) -> EndType {
    let opponent_color = turn.opposite_color();
    let possible_moves = generate_possible_moves(bb, turn, constants);

    if possible_moves.len() == 0 {
        let opponent_reach = get_reach_board(&bb, opponent_color, constants);

        let checkmate = match turn {
            ChessColor::White => (opponent_reach & bb[PieceNum::WhiteKing as usize]) == bb[PieceNum::WhiteKing as usize],
            ChessColor::Black => (opponent_reach & bb[PieceNum::BlackKing as usize]) == bb[PieceNum::BlackKing as usize]
        };

        if checkmate {
            return EndType::chess_color_to_win_type(turn.opposite_color())
        } else {
            return EndType::Tie;
        }
    }

    EndType::NoEnd
}

impl ChessBoardContract for BitBoard {
    fn generate_moves(&self, turn: ChessColor, constants: &Constants) -> Result<Vec<(String, BitBoard)>, ChessError> {
        let mut possible_moves = Vec::new();

        let board = self;
        let opposite_color = turn.opposite_color();
        let occupied_board = get_occupied_board(board);
        let own_pieces = get_full_color_board(board, turn);
        let opposite_pieces = get_full_color_board(board, opposite_color);
        let reach_board = get_reach_board(board, turn, constants);
        let opposite_reach_board = get_reach_board(board, opposite_color, constants);

        match turn {
            ChessColor::White => {
                // Pawns
                let mut tmp = board[PieceNum::WhitePawn as usize];
                while tmp != 0 {
                    let i = tmp.trailing_zeros();

                    if occupied_board & 1 << (i + 8) == 0 {
                        let mut new_board = board.clone();
                        new_board[PieceNum::WhitePawn as usize] -= 1 << i;
                        if 55 <= i + 8 && i + 8 < 64 {
                            new_board[PieceNum::WhiteQueen as usize] += 1 << (i + 8);
                        } else {
                            new_board[PieceNum::WhitePawn as usize] += 1 << (i + 8);
                        }
                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 + 8)), new_board));

                        if 7 <= i && i < 16 && occupied_board & 1 << (i + 16) == 0 {
                            let mut new_board = board.clone();
                            new_board[PieceNum::WhitePawn as usize] -= 1 << i;
                            new_board[PieceNum::WhitePawn as usize] += 1 << (i + 16);
                            possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 + 16)), new_board));
                        }
                    }

                    let mut possible_attacks = constants.pawn_reach[0][i as usize] & opposite_pieces;
                    while possible_attacks != 0 {
                        let j = possible_attacks.trailing_zeros();

                        let mut new_board = board.clone();
                        new_board[PieceNum::WhitePawn as usize] -= 1 << i;
                        new_board[PieceNum::WhitePawn as usize] += 1 << j;

                        new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                        new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                        new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                        new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                        new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                        possible_attacks -= 1 << j;
                    }

                    tmp -= 1 << i;
                }

                // Rooks
                let mut tmp = board[PieceNum::WhiteRook as usize];
                while tmp != 0 {
                    let i = tmp.trailing_zeros();

                    let reachable = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                    let mut moveable = reachable & !own_pieces;

                    while moveable != 0 {
                        let j = moveable.trailing_zeros();

                        let mut new_board = board.clone();
                        new_board[PieceNum::WhiteRook as usize] -= 1 << i;
                        new_board[PieceNum::WhiteRook as usize] += 1 << j;

                        new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                        new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                        new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                        new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                        new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                        moveable -= 1 << j;
                    }

                    tmp -= 1 << i;
                }

                // Knights
                let mut tmp = board[PieceNum::WhiteKnight as usize];
                while tmp != 0 {
                    let i = tmp.trailing_zeros();

                    let reachable = constants.knight_reach[i as usize];
                    let mut moveable = reachable & !own_pieces;

                    while moveable != 0 {
                        let j = moveable.trailing_zeros();

                        let mut new_board = board.clone();
                        new_board[PieceNum::WhiteKnight as usize] -= 1 << i;
                        new_board[PieceNum::WhiteKnight as usize] += 1 << j;

                        new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                        new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                        new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                        new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                        new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                        moveable -= 1 << j;
                    }

                    tmp -= 1 << i;
                }

                // Bishops
                let mut tmp = board[PieceNum::WhiteBishop as usize];
                while tmp != 0 {
                    let i = tmp.trailing_zeros();

                    let reachable = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                    let mut moveable = reachable & !own_pieces;

                    while moveable != 0 {
                        let j = moveable.trailing_zeros();

                        let mut new_board = board.clone();
                        new_board[PieceNum::WhiteBishop as usize] -= 1 << i;
                        new_board[PieceNum::WhiteBishop as usize] += 1 << j;

                        new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                        new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                        new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                        new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                        new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                        moveable -= 1 << j;
                    }

                    tmp -= 1 << i;
                }

                // Queen
                let mut tmp = board[PieceNum::WhiteQueen as usize];
                while tmp != 0 {
                    let i = tmp.trailing_zeros();

                    let reachable_straight = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                    let reachable_diagonals = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                    let reachable = reachable_straight | reachable_diagonals;
                    let mut moveable = reachable & !own_pieces;

                    while moveable != 0 {
                        let j = moveable.trailing_zeros();

                        let mut new_board = board.clone();
                        new_board[PieceNum::WhiteQueen as usize] -= 1 << i;
                        new_board[PieceNum::WhiteQueen as usize] += 1 << j;

                        new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                        new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                        new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                        new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                        new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                        moveable -= 1 << j;
                    }

                    tmp -= 1 << i;
                }

                // King
                let mut tmp = board[PieceNum::WhiteKing as usize];
                while tmp != 0 {
                    let i = tmp.trailing_zeros();

                    let reachable = constants.king_reach[i as usize];
                    let mut moveable = reachable & !own_pieces;

                    while moveable != 0 {
                        let j = moveable.trailing_zeros();

                        let mut new_board = board.clone();
                        new_board[PieceNum::WhiteKing as usize] -= 1 << i;
                        new_board[PieceNum::WhiteKing as usize] += 1 << j;

                        new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                        new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                        new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                        new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                        new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                        moveable -= 1 << j;
                    }

                    tmp -= 1 << i;
                }
            },
            ChessColor::Black => {
                // Pawns
                let mut tmp = board[PieceNum::BlackPawn as usize];
                while tmp != 0 {
                    let i = tmp.trailing_zeros();

                    if occupied_board & 1 << (i - 8) == 0 {
                        let mut new_board = board.clone();
                        new_board[PieceNum::BlackPawn as usize] -= 1 << i;
                        // if 0 <= i - 8 && i - 8 < 8 {
                        if i - 8 < 8 {
                            new_board[PieceNum::BlackQueen as usize] += 1 << (i - 8);
                        } else {
                            new_board[PieceNum::BlackPawn as usize] += 1 << (i - 8);
                        }
                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 - 8)), new_board));

                        if 48 <= i && i < 56 && occupied_board & 1 << (i - 16) == 0 {
                            let mut new_board = board.clone();
                            new_board[PieceNum::BlackPawn as usize] -= 1 << i;
                            new_board[PieceNum::BlackPawn as usize] += 1 << (i - 16);
                            possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 - 16)), new_board));
                        }
                    }

                    let mut possible_attacks = constants.pawn_reach[1][i as usize] & opposite_pieces;
                    while possible_attacks != 0 {
                        let j = possible_attacks.trailing_zeros();

                        let mut new_board = board.clone();
                        new_board[PieceNum::BlackPawn as usize] -= 1 << i;
                        new_board[PieceNum::BlackPawn as usize] += 1 << j;

                        new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                        new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                        possible_attacks -= 1 << j;
                    }

                    tmp -= 1 << i;
                }

                // Rooks
                let mut tmp = board[PieceNum::BlackRook as usize];
                while tmp != 0 {
                    let i = tmp.trailing_zeros();

                    let reachable = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                    let mut moveable = reachable & !own_pieces;

                    while moveable != 0 {
                        let j = moveable.trailing_zeros();

                        let mut new_board = board.clone();
                        new_board[PieceNum::BlackRook as usize] -= 1 << i;
                        new_board[PieceNum::BlackRook as usize] += 1 << j;

                        new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                        new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                        moveable -= 1 << j;
                    }

                    tmp -= 1 << i;
                }

                // Knights
                let mut tmp = board[PieceNum::BlackKnight as usize];
                while tmp != 0 {
                    let i = tmp.trailing_zeros();

                    let reachable = constants.knight_reach[i as usize];
                    let mut moveable = reachable & !own_pieces;

                    while moveable != 0 {
                        let j = moveable.trailing_zeros();

                        let mut new_board = board.clone();
                        new_board[PieceNum::BlackKnight as usize] -= 1 << i;
                        new_board[PieceNum::BlackKnight as usize] += 1 << j;

                        new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                        new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                        moveable -= 1 << j;
                    }

                    tmp -= 1 << i;
                }

                // Bishops
                let mut tmp = board[PieceNum::BlackBishop as usize];
                while tmp != 0 {
                    let i = tmp.trailing_zeros();

                    let reachable = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                    let mut moveable = reachable & !own_pieces;

                    while moveable != 0 {
                        let j = moveable.trailing_zeros();

                        let mut new_board = board.clone();
                        new_board[PieceNum::BlackBishop as usize] -= 1 << i;
                        new_board[PieceNum::BlackBishop as usize] += 1 << j;

                        new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                        new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                        moveable -= 1 << j;
                    }

                    tmp -= 1 << i;
                }

                // Queen
                let mut tmp = board[PieceNum::BlackQueen as usize];
                while tmp != 0 {
                    let i = tmp.trailing_zeros();

                    let reachable_straight = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                    let reachable_diagonals = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                    let reachable = reachable_straight | reachable_diagonals;
                    let mut moveable = reachable & !own_pieces;

                    while moveable != 0 {
                        let j = moveable.trailing_zeros();

                        let mut new_board = board.clone();
                        new_board[PieceNum::BlackQueen as usize] -= 1 << i;
                        new_board[PieceNum::BlackQueen as usize] += 1 << j;

                        new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                        new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                        moveable -= 1 << j;
                    }

                    tmp -= 1 << i;
                }

                // King
                let mut tmp = board[PieceNum::BlackKing as usize];
                while tmp != 0 {
                    let i = tmp.trailing_zeros();

                    let reachable = constants.king_reach[i as usize];
                    let mut moveable = reachable & !own_pieces;

                    while moveable != 0 {
                        let j = moveable.trailing_zeros();

                        let mut new_board = board.clone();
                        new_board[PieceNum::BlackKing as usize] -= 1 << i;
                        new_board[PieceNum::BlackKing as usize] += 1 << j;

                        new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                        new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                        new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                        moveable -= 1 << j;
                    }

                    tmp -= 1 << i;
                }
            }
        }

        // Validate
        let mut res: Vec<(String, BitBoard)> = Vec::new();
        for mov in possible_moves {
            let (mov_pos, board) = mov;

            if !is_in_check(&board, turn, constants) {
                let mov_str = format!("{}{} {}{}", get_letter(mov_pos.0.0 as usize), get_number(mov_pos.0.1 as usize), get_letter(mov_pos.1.0 as usize), get_number(mov_pos.1.1 as usize));
                res.push((mov_str, board));
            }
        }

        Ok(res)
    }

    fn check_game_end(&self, turn: ChessColor, constants: &Constants) -> Result<EndType, ChessError> {
        Ok(bitboard_check_game_end(self, turn, constants))
    }

    fn get_value_of_pieces(&self, piece_values: [i32; 6]) -> i32 {
        let mut res = 0;

        res += (self[PieceNum::WhitePawn as usize].count_ones() as i32) * piece_values[PieceType::Pawn as usize];
        res += (self[PieceNum::WhiteRook as usize].count_ones() as i32) * piece_values[PieceType::Rook as usize];
        res += (self[PieceNum::WhiteKnight as usize].count_ones() as i32) * piece_values[PieceType::Knight as usize];
        res += (self[PieceNum::WhiteBishop as usize].count_ones() as i32) * piece_values[PieceType::Bishop as usize];
        res += (self[PieceNum::WhiteQueen as usize].count_ones() as i32) * piece_values[PieceType::Queen as usize];
        res += (self[PieceNum::WhiteKing as usize].count_ones() as i32) * piece_values[PieceType::King as usize];

        res -= (self[PieceNum::BlackPawn as usize].count_ones() as i32) * piece_values[PieceType::Pawn as usize];
        res -= (self[PieceNum::BlackRook as usize].count_ones() as i32) * piece_values[PieceType::Rook as usize];
        res -= (self[PieceNum::BlackKnight as usize].count_ones() as i32) * piece_values[PieceType::Knight as usize];
        res -= (self[PieceNum::BlackBishop as usize].count_ones() as i32) * piece_values[PieceType::Bishop as usize];
        res -= (self[PieceNum::BlackQueen as usize].count_ones() as i32) * piece_values[PieceType::Queen as usize];
        res -= (self[PieceNum::BlackKing as usize].count_ones() as i32) * piece_values[PieceType::King as usize];

        res
    }

    fn new_board() -> Self {
        generate_start_board()
    }

    fn board_ascii(&self, use_unicode: bool) -> String {
        get_bitboard_ascii(self, use_unicode)
    }

}
