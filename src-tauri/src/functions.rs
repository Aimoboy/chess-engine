
use crate::board_types::{
    normalboard::NormalBoard
};

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

pub fn fen_to_normalboard(fen: &str) -> NormalBoard {
    NormalBoard::new_start_board()
}
