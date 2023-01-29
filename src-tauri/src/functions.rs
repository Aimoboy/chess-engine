
use crate::{board_types::{
    normalboard::{NormalBoard, ChessPiece}
}, enums::{chess_color::ChessColor, piece_type::PieceType}};

pub fn get_letter(letter: usize) -> char {
    match letter {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        _ => 'h',
    }
}

pub fn get_number(number: usize) -> char {
    match number {
        0 => '1',
        1 => '2',
        2 => '3',
        3 => '4',
        4 => '5',
        5 => '6',
        6 => '7',
        _ => '8',
    }
}

pub fn pos_to_num(letter: u64, number: u64) -> u64 {
    letter + (number << 3)
}

pub fn num_to_pos(num: u64) -> (u64, u64) {
    (num & 7, num >> 3)
}

pub fn validate_move_string(move_str: &String) -> bool {
    let valid_letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let valid_numbers = ['1', '2', '3', '4', '5', '6', '7', '8'];

    let mut characters = move_str.chars();

    if !valid_letters.contains(&mut characters.nth(0).unwrap()) {
        return false;
    }

    if !valid_numbers.contains(&mut characters.nth(0).unwrap()) {
        return false;
    }

    if characters.nth(0).unwrap() != ' ' {
        return false;
    }

    if !valid_letters.contains(&mut characters.nth(0).unwrap()) {
        return false;
    }

    if !valid_numbers.contains(&mut characters.nth(0).unwrap()) {
        return false;
    }

    true
}

pub fn parse_fen_to_normalboard(fen: &str) -> (NormalBoard, ChessColor) {
    let mut split = fen.split_whitespace();

    let board_string = split.nth(0).expect("FEN board part");
    let turn_string = split.nth(0).expect("FEN turn part");
    let castle_string = split.nth(0).expect("FEN castle part");
    let en_passant_string = split.nth(0).expect("FEN en passant part");
    let half_move_string = split.nth(0).expect("FEN half move part");
    let full_move_string = split.nth(0).expect("FEN full move part");

    let mut board = NormalBoard::new_empty_board();

    let mut row = 7;
    let mut col = 0;
    for character in board_string.chars() {
        match character {
            'P' => {
                board.set_piece(col, row, Some(ChessPiece::new(PieceType::Pawn, ChessColor::White)).as_ref()).expect("White pawn");
                col += 1;
            },
            'R' => {
                board.set_piece(col, row, Some(ChessPiece::new(PieceType::Rook, ChessColor::White)).as_ref()).expect("White pawn");
                col += 1;
            },
            'N' => {
                board.set_piece(col, row, Some(ChessPiece::new(PieceType::Knight, ChessColor::White)).as_ref()).expect("White pawn");
                col += 1;
            },
            'B' => {
                board.set_piece(col, row, Some(ChessPiece::new(PieceType::Bishop, ChessColor::White)).as_ref()).expect("White pawn");
                col += 1;
            },
            'Q' => {
                board.set_piece(col, row, Some(ChessPiece::new(PieceType::Queen, ChessColor::White)).as_ref()).expect("White pawn");
                col += 1;
            },
            'K' => {
                board.set_piece(col, row, Some(ChessPiece::new(PieceType::King, ChessColor::White)).as_ref()).expect("White pawn");
                col += 1;
            },
            'p' => {
                board.set_piece(col, row, Some(ChessPiece::new(PieceType::Pawn, ChessColor::Black)).as_ref()).expect("White pawn");
                col += 1;
            },
            'r' => {
                board.set_piece(col, row, Some(ChessPiece::new(PieceType::Rook, ChessColor::Black)).as_ref()).expect("White pawn");
                col += 1;
            },
            'n' => {
                board.set_piece(col, row, Some(ChessPiece::new(PieceType::Knight, ChessColor::Black)).as_ref()).expect("White pawn");
                col += 1;
            },
            'b' => {
                board.set_piece(col, row, Some(ChessPiece::new(PieceType::Bishop, ChessColor::Black)).as_ref()).expect("White pawn");
                col += 1;
            },
            'q' => {
                board.set_piece(col, row, Some(ChessPiece::new(PieceType::Queen, ChessColor::Black)).as_ref()).expect("White pawn");
                col += 1;
            },
            'k' => {
                board.set_piece(col, row, Some(ChessPiece::new(PieceType::King, ChessColor::Black)).as_ref()).expect("White pawn");
                col += 1;
            },
            '/' => {
                col = 0;
                row -= 1;
            },
            c => {
                let digit = c.to_digit(10).expect("FEN char to digit conversion") as i32;
                col += digit;
            }
        }
    }

    if en_passant_string != "-" {
        let mut en_passant_iter = en_passant_string.chars();

        let letter = match en_passant_iter.nth(0).expect("FEN en passant letter") {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            _ => 7,
        };

        let number: i32 = en_passant_iter.nth(0).expect("FEN en passant number").to_digit(10).expect("FEN en passant number to digit") as i32 - 1;

        board.set_en_passant(Some((letter, number)));
    } else {
        board.set_en_passant(None);
    }

    board.set_half_moves(half_move_string.parse().expect("FEN half moves"));
    board.set_full_moves(full_move_string.parse().expect("FEN half moves"));

    board.set_white_left_castle(castle_string.contains("Q"));
    board.set_white_right_castle(castle_string.contains("K"));
    board.set_black_left_castle(castle_string.contains("q"));
    board.set_black_right_castle(castle_string.contains("k"));

    for i in 0..8 {
        for j in 0..8 {
            if let Ok(Some(piece)) = board.get_piece(i, j) {
                if piece.typ == PieceType::King {
                    board.set_king_pos(piece.color, (i as usize, j as usize))
                }
            }
        }
    }

    let turn = match turn_string {
        "w" => ChessColor::White,
        _ => ChessColor::Black
    };

    (board, turn)
}

pub fn normalboard_to_fen(board: &NormalBoard, turn: ChessColor) -> String {
    let mut fen = String::new();

    let mut none_counter = 0;
    for row in (0..8).rev() {
        for col in 0..8 {
            let piece = board.get_piece(col, row).expect("Normalboard to FEN piece get");

            match piece {
                Some(piece) => {
                    if none_counter > 0 {
                        fen.push(char::from_digit(none_counter, 10).expect("None counter to digit"));
                        none_counter = 0;
                    }

                    let mut piece_letter = match piece.typ {
                        PieceType::Pawn => 'p',
                        PieceType::Rook => 'r',
                        PieceType::Knight => 'n',
                        PieceType::Bishop => 'b',
                        PieceType::Queen => 'q',
                        PieceType::King => 'k',
                    };

                    if piece.color == ChessColor::White {
                        piece_letter = piece_letter.to_ascii_uppercase();
                    }

                    fen.push(piece_letter);
                },
                None => {
                    none_counter += 1
                }
            }
        }

        if row != 0 {
            if none_counter != 0 {
                fen.push(char::from_digit(none_counter, 10).expect("None counter to digit"));
                none_counter = 0;
            }
            fen.push('/');
        }
    }

    fen.push(' ');

    match turn {
        ChessColor::White => {
            fen.push('w');
        },
        ChessColor::Black => {
            fen.push('b');
        }
    }

    fen.push(' ');

    if !board.get_white_right_castle() && !board.get_white_left_castle() && !board.get_black_right_castle() && !board.get_black_left_castle() {
        fen.push('-');
    } else {
        if board.get_white_right_castle() {
            fen.push('K');
        }

        if board.get_white_left_castle() {
            fen.push('Q');
        }

        if board.get_black_right_castle() {
            fen.push('k');
        }

        if board.get_black_left_castle() {
            fen.push('q');
        }
    }

    fen.push(' ');

    match board.get_en_passant() {
        None => {
            fen.push('-');
        },
        Some((letter_num, num)) => {
            let letter = match letter_num {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                5 => 'f',
                6 => 'g',
                _ => 'h',
            };

            let num_char = char::from_digit(num as u32 + 1, 10).expect("Normalboard en passant num to char");
            fen.push(letter);
            fen.push(num_char);
        }
    }

    fen.push(' ');

    fen.push_str(board.get_half_moves().to_string().as_str());

    fen.push(' ');

    fen.push_str(board.get_full_moves().to_string().as_str());

    fen
}
