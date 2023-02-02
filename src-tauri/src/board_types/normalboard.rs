use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::enums::{
    chess_color::ChessColor,
    end_type::EndType,
    piece_type::PieceType,
    chess_error::ChessError
};

use crate::traits::{
    chess_board_contract::ChessBoardContract
};

use crate::functions::{
    get_letter,
    get_number
};

use crate::board_types::bitboard::Constants;



pub type Pos = (usize, usize);
type ReachBoard = [[bool; 8]; 8];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChessPiece {
    pub typ: PieceType,
    pub color: ChessColor
}

impl ChessPiece {
    pub fn new(typ: PieceType, color: ChessColor) -> Self {
        Self {
            typ,
            color
        }
    }

    // Maybe this and the next function should just use self?
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

    fn piece_to_unicode(piece: &ChessPiece) -> char {
        match piece.color {
            ChessColor::White => match piece.typ {
                                    PieceType::Pawn => '\u{265F}',
                                    PieceType::Rook => '\u{265C}',
                                    PieceType::Knight => '\u{265E}',
                                    PieceType::Bishop => '\u{265D}',
                                    PieceType::Queen => '\u{265B}',
                                    PieceType::King => '\u{265A}'
                                }
            ChessColor::Black => match piece.typ {
                                    PieceType::Pawn => '\u{2659}',
                                    PieceType::Rook => '\u{2656}',
                                    PieceType::Knight => '\u{2658}',
                                    PieceType::Bishop => '\u{2657}',
                                    PieceType::Queen => '\u{2655}',
                                    PieceType::King => '\u{2654}'
                                }
        }
    }
}

pub struct NormalBoardIter<'a> {
    pieces: Vec<(Pos, &'a ChessPiece)>,
    pos: usize
}

impl<'a> NormalBoardIter<'a> {
    pub fn new(board: &'a NormalBoard) -> Self {
        let mut pieces = Vec::new();

        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = &board.board[i][j]  {
                    pieces.push(((i, j), piece))
                }
            }
        }

        Self {
            pieces,
            pos: 0
        }
    }
}

impl<'a> Iterator for NormalBoardIter<'a> {
    type Item = (Pos, &'a ChessPiece);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.pieces.len() {
            return None;
        }

        let piece = self.pieces[self.pos];
        self.pos += 1;
        return Some(piece);
    }
}

// Letter is first index then number
#[derive(Debug, Clone)]
pub struct NormalBoard {
    board: [[Option<ChessPiece>; 8]; 8],
    en_passant: Option<(i32, i32)>,
    white_left_castle: bool,
    white_right_castle: bool,
    black_left_castle: bool,
    black_right_castle: bool,
    half_moves_since_piece_capture_or_pawn_advance: i32,
    full_move_counter: i32,
    white_king_pos: Pos,
    black_king_pos: Pos
}

impl Hash for NormalBoard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.hash(state);
        self.white_left_castle.hash(state);
        self.white_right_castle.hash(state);
        self.black_left_castle.hash(state);
        self.black_right_castle.hash(state);
    }
}

impl PartialEq for NormalBoard {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..8 {
            for j in 0..8 {
                let piece = if let Ok(val) = self.get_piece(i, j) {
                    val
                } else {
                    return false;
                };

                let other_piece = if let Ok(val) = other.get_piece(i, j) {
                    val
                } else {
                    return false;
                };

                if piece.is_some() && other_piece.is_none() || piece.is_none() && other_piece.is_some() {
                    return false;
                }

                if let Some(piece) = piece {
                    if let Some(other_piece) = other_piece {
                        if piece.typ != other_piece.typ || piece.color != other_piece.color {
                            return false;
                        }
                    }
                }
            }
        }

        self.white_left_castle == other.white_left_castle && self.white_right_castle == other.white_right_castle && self.black_left_castle == other.black_left_castle && self.black_right_castle == other.black_right_castle
    }
}

impl Eq for NormalBoard { }

impl NormalBoard {
    pub fn set_en_passant(&mut self, val: Option<(i32, i32)>) {
        self.en_passant = val;
    }

    pub fn set_white_left_castle(&mut self, val: bool) {
        self.white_left_castle = val;
    }

    pub fn set_white_right_castle(&mut self, val: bool) {
        self.white_right_castle = val;
    }

    pub fn set_black_left_castle(&mut self, val: bool) {
        self.black_left_castle = val;
    }

    pub fn set_black_right_castle(&mut self, val: bool) {
        self.black_right_castle = val;
    }

    pub fn set_half_moves(&mut self, val: i32) {
        self.half_moves_since_piece_capture_or_pawn_advance = val;
    }

    pub fn set_full_moves(&mut self, val: i32) {
        self.full_move_counter = val;
    }

    pub fn get_en_passant(&self) -> Option<(i32, i32)> {
        return self.en_passant;
    }

    pub fn get_white_left_castle(&self) -> bool {
        return self.white_left_castle;
    }

    pub fn get_white_right_castle(&self) -> bool {
        return self.white_right_castle;
    }

    pub fn get_black_left_castle(&self) -> bool {
        return self.black_left_castle;
    }

    pub fn get_black_right_castle(&self) -> bool {
        return self.black_right_castle;
    }

    pub fn get_half_moves(&self) -> i32 {
        return self.half_moves_since_piece_capture_or_pawn_advance;
    }

    pub fn get_full_moves(&self) -> i32 {
        return self.full_move_counter;
    }

    pub fn set_white_king_pos(&mut self, pos: Pos) {
        self.white_king_pos = pos;
    }

    pub fn set_black_king_pos(&mut self, pos: Pos) {
        self.black_king_pos = pos;
    }

    pub fn get_white_king_pos(&self) -> Pos {
        return self.white_king_pos;
    }

    pub fn get_black_king_pos(&self) -> Pos {
        return self.black_king_pos;
    }

    pub fn iter(&self) -> NormalBoardIter {
        NormalBoardIter::new(self)
    }

    pub fn get_piece(&self, letter: i32, number: i32) -> Result<Option<&ChessPiece>, ChessError> {
        if letter < 0 || letter > 7 || number < 0 || number > 7 {
            return Err(ChessError::OutsideBounds);
        }

        Ok(self.board[letter as usize][number as usize].as_ref())
    }

    pub fn set_piece(&mut self, letter: i32, number: i32, piece: Option<&ChessPiece>) -> Result<bool, ChessError> {
        if letter < 0 || letter > 7 || number < 0 || number > 7 {
            return Err(ChessError::OutsideBounds);
        }

        let new_piece = match piece {
            Some(val) => Some(val.clone()),
            None => None
        };

        self.board[letter as usize][number as usize] = new_piece;

        Ok(true)
    }

    pub fn move_piece(&mut self, from_letter: i32, from_number: i32, to_letter: i32, to_number: i32) -> Result<bool, ChessError> {
        let piece = self.get_piece(from_letter, from_number)?;

        match piece {
            None => {
                return Err(ChessError::InvalidMove);
            },
            Some(piece) => {
                let piece = piece.clone();
                self.set_piece(to_letter, to_number, Some(&piece))?;
                self.delete_piece(from_letter, from_number)?;
            }
        }

        Ok(true)
    }

    pub fn delete_piece(&mut self, letter: i32, number: i32) -> Result<bool, ChessError> {
        if letter < 0 || letter > 7 || number < 0 || number > 7 {
            return Err(ChessError::OutsideBounds);
        }

        self.board[letter as usize][number as usize] = None;

        Ok(true)
    }

    fn blank_board() -> [[Option<ChessPiece>; 8]; 8] {
        [[None, None, None, None, None, None, None, None],
         [None, None, None, None, None, None, None, None],
         [None, None, None, None, None, None, None, None],
         [None, None, None, None, None, None, None, None],
         [None, None, None, None, None, None, None, None],
         [None, None, None, None, None, None, None, None],
         [None, None, None, None, None, None, None, None],
         [None, None, None, None, None, None, None, None]]
    }

    pub fn new_empty_board() -> Self {
        Self {
            board: Self::blank_board(),
            en_passant: None,
            white_left_castle: false,
            white_right_castle: false,
            black_left_castle: false,
            black_right_castle: false,
            half_moves_since_piece_capture_or_pawn_advance: 0,
            full_move_counter: 1,
            white_king_pos: (8, 8),
            black_king_pos: (8, 8)
        }
    }

    pub fn new_start_board() -> Self {
        let mut board = Self::blank_board();

        board[0][0] = Some(ChessPiece::new(PieceType::Rook, ChessColor::White));
        board[1][0] = Some(ChessPiece::new(PieceType::Knight, ChessColor::White));
        board[2][0] = Some(ChessPiece::new(PieceType::Bishop, ChessColor::White));
        board[3][0] = Some(ChessPiece::new(PieceType::Queen, ChessColor::White));
        board[4][0] = Some(ChessPiece::new(PieceType::King, ChessColor::White));
        board[5][0] = Some(ChessPiece::new(PieceType::Bishop, ChessColor::White));
        board[6][0] = Some(ChessPiece::new(PieceType::Knight, ChessColor::White));
        board[7][0] = Some(ChessPiece::new(PieceType::Rook, ChessColor::White));

        for i in 0..8 {
            board[i][1] = Some(ChessPiece::new(PieceType::Pawn, ChessColor::White));
        }

        for i in 0..8 {
            board[i][6] = Some(ChessPiece::new(PieceType::Pawn, ChessColor::Black));
        }

        board[0][7] = Some(ChessPiece::new(PieceType::Rook, ChessColor::Black));
        board[1][7] = Some(ChessPiece::new(PieceType::Knight, ChessColor::Black));
        board[2][7] = Some(ChessPiece::new(PieceType::Bishop, ChessColor::Black));
        board[3][7] = Some(ChessPiece::new(PieceType::Queen, ChessColor::Black));
        board[4][7] = Some(ChessPiece::new(PieceType::King, ChessColor::Black));
        board[5][7] = Some(ChessPiece::new(PieceType::Bishop, ChessColor::Black));
        board[6][7] = Some(ChessPiece::new(PieceType::Knight, ChessColor::Black));
        board[7][7] = Some(ChessPiece::new(PieceType::Rook, ChessColor::Black));

        Self {
            board: board,
            en_passant: None,
            white_left_castle: true,
            white_right_castle: true,
            black_left_castle: true,
            black_right_castle: true,
            half_moves_since_piece_capture_or_pawn_advance: 0,
            full_move_counter: 1,
            white_king_pos: (4, 0),
            black_king_pos: (4, 7)
        }
    }

    pub fn board_ascii(&self, use_unicode: bool) -> String {
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
                string.push_str("| ");
                string.push(match &self.board[j][i] {
                    Some(piece) => if use_unicode {
                        ChessPiece::piece_to_unicode(&piece)
                    } else {
                        ChessPiece::piece_to_char(&piece)
                    },
                    None => ' '
                });
                string.push(' ');

                match &self.board[j][i] {
                    Some(piece) => {
                        if piece.typ != PieceType::Pawn || piece.color != ChessColor::White {
                            string.push(' ');
                        }
                    },
                    None => string.push(' ')
                }
            }
            string.push_str("|\n");
        }
        if use_unicode {
            string.push_str("  +----+----+----+----+----+----+----+----+\n");
        } else {
            string.push_str("  +---+---+---+---+---+---+---+---+\n");
        }

        if use_unicode {
            string.push_str("    A    B    C    D    E    F    G    H");
        } else {
            string.push_str("    A   B   C   D   E   F   G   H");
        }

        string
    }

    pub fn generate_reachable_tiles_board(&self, color: ChessColor) -> ReachBoard {
        let mut reach_board = [[false; 8]; 8];

        for (pos, piece) in self.iter() {
            if piece.color == color {
                match piece.typ {
                    PieceType::Pawn => Self::generate_pawn_reach(self, piece, &mut reach_board, pos),
                    PieceType::Rook => Self::generate_rook_reach(self, &mut reach_board, pos),
                    PieceType::Bishop => Self::generate_bishop_reach(self, &mut reach_board, pos),
                    PieceType::Knight => Self::generate_knight_reach(self, &mut reach_board, pos),
                    PieceType::Queen => Self::generate_queen_reach(self, &mut reach_board, pos),
                    PieceType::King => Self::generate_king_reach(self, &mut reach_board, pos)
                };
            }
        }

        reach_board
    }

    fn generate_pawn_reach(board: &NormalBoard, piece: &ChessPiece, reach_board: &mut ReachBoard, pos: Pos) {
        let (letter, number) = pos;
        let side_const = piece.color.side_const();

        let directions = [-1, 1];

        // Left, right
        for dir in directions {
            if let Ok(_) = board.get_piece(letter as i32 + dir, number as i32 + side_const) {
                reach_board[(letter as i32 + dir) as usize][(number as i32 + side_const) as usize] = true;
            }
        }
    }

    fn generate_rook_reach(board: &NormalBoard, reach_board: &mut ReachBoard, pos: Pos) {
        let (letter, number) = pos;

        // Up, right, down, left
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for dir in directions {
            let (letter_const, number_const) = dir;
            for i in 1..8 {
                let new_letter = letter as i32 + i * letter_const;
                let new_number = number as i32 + i * number_const;

                match board.get_piece(new_letter, new_number) {
                    Ok(Some(_)) => {
                        reach_board[new_letter as usize][new_number as usize] = true;
                        break;
                    },
                    Ok(_) => (),
                    Err(_) => {
                        break;
                    }
                }

                if let Ok(Some(_)) = board.get_piece(new_letter, new_number) {
                    reach_board[new_letter as usize][new_number as usize] = true;
                    break;
                }

                reach_board[new_letter as usize][new_number as usize] = true;
            }
        }
    }

    fn generate_bishop_reach(board: &NormalBoard, reach_board: &mut ReachBoard, pos: Pos) {
        let (letter, number) = pos;

        // Up-right, down-left, down-right, up-left
        let directions = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

        for dir in directions {
            let (letter_const, number_const) = dir;
            for i in 1..8 {
                let new_letter = letter as i32 + i * letter_const;
                let new_number = number as i32 + i * number_const;

                match board.get_piece(new_letter, new_number) {
                    Ok(Some(_)) => {
                        reach_board[new_letter as usize][new_number as usize] = true;
                        break;
                    },
                    Ok(_) => (),
                    Err(_) => {
                        break;
                    }
                }
            }
        }
    }

    fn generate_knight_reach(board: &NormalBoard, reach_board: &mut ReachBoard, pos: Pos) {
        let (letter, number) = pos;

        // Clockwise starting from up-right
        let directions = [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)];

        for dir in directions {
        let (letter_const, number_const) = dir;
            let new_letter = letter as i32 + letter_const;
            let new_number = number as i32 + number_const;

            if let Ok(_) = board.get_piece(new_letter, new_number) {
                reach_board[new_letter as usize][new_number as usize] = true;
            }
        }
    }

    fn generate_queen_reach(board: &NormalBoard, reach_board: &mut ReachBoard, pos: Pos) {
        Self::generate_rook_reach(board, reach_board, pos);
        Self::generate_bishop_reach(board, reach_board, pos);
    }

    fn generate_king_reach(board: &NormalBoard, reach_board: &mut ReachBoard, pos: Pos) {
        let (letter, number) = pos;

        // Clockwise starting from up-right
        let directions = [(1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1)];

        for dir in directions {
        let (letter_const, number_const) = dir;
            let new_letter = letter as i32 + letter_const;
            let new_number = number as i32 + number_const;

            if let Ok(_) = board.get_piece(new_letter, new_number) {
                reach_board[new_letter as usize][new_number as usize] = true;
            }
        }
    }

    pub fn generate_possible_moves(&self, turn: ChessColor) -> Result<Vec<(String, NormalBoard)>, ChessError> {
        let res = self.iter()
            .filter(|(_, piece)| piece.color == turn)
            .map(|(pos, piece)| self.generate_possible_moves_for_piece(piece, turn, pos))
            .fold(Ok(Vec::with_capacity(50)), |acc, item| {
                let mut acc_val: Vec<(String, NormalBoard)> = acc?;
                let mut item_val = item?;
                acc_val.append(&mut item_val);

                Ok(acc_val)
            });
        
        Ok(res?)
    }

    fn generate_possible_moves_for_piece(&self, piece: &ChessPiece, turn: ChessColor, pos: Pos) -> Result<Vec<(String, NormalBoard)>, ChessError> {
        Ok(match piece.typ {
            PieceType::Pawn => self.generate_possible_pawn_moves(piece, pos),
            PieceType::Rook => self.generate_possible_rook_moves(piece, pos),
            PieceType::Knight => self.generate_possible_knight_moves(piece, pos),
            PieceType::Bishop => self.generate_possible_bishop_moves(piece, pos),
            PieceType::Queen => self.generate_possible_queen_moves(piece, pos),
            PieceType::King => self.generate_possible_king_moves(piece, pos)
        }?.into_iter().filter(|(_, board)| Self::check_if_valid_move(board, turn)).collect())
    }

    fn check_if_valid_move(board: &NormalBoard, turn: ChessColor) -> bool {
        let opponent_reach_board = board.generate_reachable_tiles_board(turn.opposite_color());
        let (king_pos_letter, king_pos_number) = board.get_king_pos(turn);

        !opponent_reach_board[king_pos_letter][king_pos_number]
    }

    fn generate_possible_pawn_moves(&self, piece: &ChessPiece, pos: Pos) -> Result<Vec<(String, NormalBoard)>, ChessError> {
        let (letter, number) = pos;
        let side_const = piece.color.side_const();

        let mut possible_moves = Vec::new();

        // One step
        let new_letter = letter as i32;
        let new_number = number as i32 + side_const;
        if let Ok(None) = self.get_piece(letter as i32, new_number) {
            // Handle Promotion
            if piece.color == ChessColor::White && new_number == 7 || piece.color == ChessColor::Black && new_number == 0 {
                let promotion_types = [PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen];

                for promotion_type in promotion_types {
                    let mut new_board = self.clone();

                    if piece.color == ChessColor::Black {
                        new_board.set_full_moves(new_board.get_full_moves() + 1);
                    }

                    new_board.delete_piece(letter as i32, number as i32)?;
                    let piece = ChessPiece::new(promotion_type, piece.color);
                    new_board.set_piece(new_letter, new_number, Some(piece).as_ref())?;
                    new_board.set_en_passant(None);
                    new_board.set_half_moves(0);

                    let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                    possible_moves.push((mov_str, new_board));
                }
            } else {
                let mut new_board = self.clone();
                
                if piece.color == ChessColor::Black {
                    new_board.set_full_moves(new_board.get_full_moves() + 1);
                }

                new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;
                new_board.set_en_passant(None);
                new_board.set_half_moves(0);

                let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                possible_moves.push((mov_str, new_board));
            }
        }

        // Double step
        if piece.color == ChessColor::White && number == 1 || piece.color == ChessColor::Black && number == 6 {
            let new_letter = letter as i32;
            let new_number = number as i32 + side_const;
            if let Ok(None) = self.get_piece(letter as i32, new_number) {
                let new_number = number as i32 + side_const * 2;
                if let Ok(None) = self.get_piece(letter as i32, new_number) {
                    let mut new_board = self.clone();

                    if piece.color == ChessColor::Black {
                        new_board.set_full_moves(new_board.get_full_moves() + 1);
                    }

                    new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;
                    new_board.set_half_moves(0);

                    let en_passant_coords = (letter as i32, new_number as i32 - side_const);
                    new_board.set_en_passant(Some(en_passant_coords));

                    let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                    possible_moves.push((mov_str, new_board));
                }
            }
        }

        // Left, right
        let direction = [-1, 1];

        // Diagonal attack
        for dir in direction {
            let new_letter = letter as i32 + dir;
            let new_number = number as i32 + side_const;

            if let Ok(Some(other_piece)) = self.get_piece(new_letter, new_number) {
                if piece.color != other_piece.color {
                    // Handle Promotion
                    if piece.color == ChessColor::White && new_number == 7 || piece.color == ChessColor::Black && new_number == 0 {
                        let promotion_types = [PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen];

                        for promotion_type in promotion_types {
                            let mut new_board = self.clone();

                            if piece.color == ChessColor::Black {
                                new_board.set_full_moves(new_board.get_full_moves() + 1);
                            }

                            new_board.delete_piece(letter as i32, number as i32)?;
                            let piece = ChessPiece::new(promotion_type, piece.color);
                            new_board.set_piece(new_letter, new_number, Some(piece).as_ref())?;
                            new_board.set_en_passant(None);
                            new_board.set_half_moves(0);

                            let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                            possible_moves.push((mov_str, new_board));
                        }
                    } else {
                        let mut new_board = self.clone();

                        if piece.color == ChessColor::Black {
                            new_board.set_full_moves(new_board.get_full_moves() + 1);
                        }

                        new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;
                        new_board.set_en_passant(None);
                        new_board.set_half_moves(0);

                        let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                        possible_moves.push((mov_str, new_board));
                    }
                }
            }

            // En passant
            if let Some((en_passant_letter, en_passant_number)) = self.get_en_passant() {
                if new_letter == en_passant_letter && new_number == en_passant_number {
                    let mut new_board = self.clone();

                    if piece.color == ChessColor::Black {
                        new_board.set_full_moves(new_board.get_full_moves() + 1);
                    }

                    new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;
                    new_board.delete_piece(new_letter, new_number - side_const)?;
                    new_board.set_en_passant(None);
                    new_board.set_half_moves(0);

                    let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                    possible_moves.push((mov_str, new_board));
                }
            }
        }

        Ok(possible_moves)
    }

    fn generate_possible_rook_moves(&self, piece: &ChessPiece, pos: Pos) -> Result<Vec<(String, NormalBoard)>, ChessError> {
        let (letter, number) = pos;

        let mut possible_moves = Vec::new();

        // Up, right, down, left
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for dir in directions {
            let (letter_const, number_const) = dir;
            for i in 1..8 {
                let new_letter = letter as i32 + i * letter_const;
                let new_number = number as i32 + i * number_const;
                
                match self.get_piece(new_letter, new_number) {
                    Ok(None) => {
                        let mut new_board = self.clone();

                        if piece.color == ChessColor::Black {
                            new_board.set_full_moves(new_board.get_full_moves() + 1);
                        }

                        new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;
                        new_board.set_en_passant(None);
                        new_board.set_half_moves(self.get_half_moves() + 1);

                        if letter == 0 && number == 0 && piece.color == ChessColor::White {
                            new_board.set_white_left_castle(false);
                        }

                        if letter == 7 && number == 0 && piece.color == ChessColor::White {
                            new_board.set_white_right_castle(false);
                        }

                        if letter == 0 && number == 7 && piece.color == ChessColor::Black {
                            new_board.set_black_left_castle(false);
                        }

                        if letter == 7 && number == 7 && piece.color == ChessColor::Black {
                            new_board.set_black_right_castle(false);
                        }

                        let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                        possible_moves.push((mov_str, new_board));
                    },
                    Ok(Some(other_piece)) => {
                        if piece.color != other_piece.color {
                            let mut new_board = self.clone();

                            if piece.color == ChessColor::Black {
                                new_board.set_full_moves(new_board.get_full_moves() + 1);
                            }

                            new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;
                            new_board.set_en_passant(None);
                            new_board.set_half_moves(0);

                            if letter == 0 && number == 0 && piece.color == ChessColor::White {
                                new_board.set_white_left_castle(false);
                            }
    
                            if letter == 7 && number == 0 && piece.color == ChessColor::White {
                                new_board.set_white_right_castle(false);
                            }
    
                            if letter == 0 && number == 7 && piece.color == ChessColor::Black {
                                new_board.set_black_left_castle(false);
                            }
    
                            if letter == 7 && number == 7 && piece.color == ChessColor::Black {
                                new_board.set_black_right_castle(false);
                            }

                            let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                            possible_moves.push((mov_str, new_board));
                        }
                        break;
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        }

        Ok(possible_moves)
    }

    fn generate_possible_knight_moves(&self, piece: &ChessPiece, pos: Pos) -> Result<Vec<(String, NormalBoard)>, ChessError> {
        let (letter, number) = pos;

        let mut possible_moves = Vec::new();

        // Clockwise starting from up-right
        let directions = [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)];

        for dir in directions {
            let (letter_const, number_const) = dir;
            let new_letter = letter as i32 + letter_const;
            let new_number = number as i32 + number_const;

            match self.get_piece(new_letter, new_number) {
                Ok(None) => {
                    let mut new_board = self.clone();

                    if piece.color == ChessColor::Black {
                        new_board.set_full_moves(new_board.get_full_moves() + 1);
                    }

                    new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;
                    new_board.set_en_passant(None);
                    new_board.set_half_moves(self.get_half_moves() + 1);

                    let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                    possible_moves.push((mov_str, new_board));
                },
                Ok(Some(other_piece)) => {
                    if piece.color != other_piece.color {
                        let mut new_board = self.clone();

                        if piece.color == ChessColor::Black {
                            new_board.set_full_moves(new_board.get_full_moves() + 1);
                        }

                        new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;
                        new_board.set_en_passant(None);
                        new_board.set_half_moves(0);

                        let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                        possible_moves.push((mov_str, new_board));
                    }
                },
                Err(_) => ()
            }
        }

        Ok(possible_moves)
    }

    fn generate_possible_bishop_moves(&self, piece: &ChessPiece, pos: Pos) -> Result<Vec<(String, NormalBoard)>, ChessError> {
        let (letter, number) = pos;

        let mut possible_moves = Vec::new();

        // Up-right, down-left, down-right, up-left
        let directions = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

        for dir in directions {
            let (letter_const, number_const) = dir;
            for i in 1..8 {
                let new_letter = letter as i32 + i * letter_const;
                let new_number = number as i32 + i * number_const;

                match self.get_piece(new_letter, new_number) {
                    Ok(None) => {
                        let mut new_board = self.clone();

                        if piece.color == ChessColor::Black {
                            new_board.set_full_moves(new_board.get_full_moves() + 1);
                        }

                        new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;
                        new_board.set_en_passant(None);
                        new_board.set_half_moves(self.get_half_moves() + 1);

                        let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                        possible_moves.push((mov_str, new_board));
                    },
                    Ok(Some(other_piece)) => {
                        if piece.color != other_piece.color {
                            let mut new_board = self.clone();

                            if piece.color == ChessColor::Black {
                                new_board.set_full_moves(new_board.get_full_moves() + 1);
                            }

                            new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;
                            new_board.set_en_passant(None);
                            new_board.set_half_moves(0);
    
                            let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                            possible_moves.push((mov_str, new_board));
                        }
                        break;
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        }

        Ok(possible_moves)
    }

    fn generate_possible_queen_moves(&self, piece: &ChessPiece, pos: Pos) -> Result<Vec<(String, NormalBoard)>, ChessError> {
        let mut possible_moves = Vec::new();

        possible_moves.append(&mut self.generate_possible_rook_moves(piece, pos)?);
        possible_moves.append(&mut self.generate_possible_bishop_moves(piece, pos)?);

        Ok(possible_moves)
    }

    fn generate_possible_king_moves(&self, piece: &ChessPiece, pos: Pos) -> Result<Vec<(String, NormalBoard)>, ChessError> {
        let (letter, number) = pos;

        let mut possible_moves = Vec::new();

        // Clockwise starting from up-right
        let directions = [(1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1)];

        for dir in directions {
            let (letter_const, number_const) = dir;
            let new_letter = letter as i32 + letter_const;
            let new_number = number as i32 + number_const;

            
            let generate_move = match self.get_piece(new_letter, new_number) {
                Ok(None) => {
                    true
                },
                Ok(Some(other_piece)) => {
                    piece.color != other_piece.color
                },
                Err(_) => false
            };

            if generate_move {
                let mut new_board = self.clone();

                if piece.color == ChessColor::Black {
                    new_board.set_full_moves(new_board.get_full_moves() + 1);
                }

                new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;
                new_board.set_en_passant(None);
                new_board.set_king_pos(piece.color, (new_letter as usize, new_number as usize));

                if let Ok(Some(_)) = self.get_piece(new_letter, new_number) {
                    new_board.set_half_moves(0);
                } else {
                    new_board.set_half_moves(self.get_half_moves() + 1);
                }

                match piece.color {
                    ChessColor::White => {
                        new_board.set_white_left_castle(false);
                        new_board.set_white_right_castle(false);
                    },
                    ChessColor::Black => {
                        new_board.set_black_left_castle(false);
                        new_board.set_black_right_castle(false);
                    }
                }

                let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                possible_moves.push((mov_str, new_board));
            }
        }

        let opponent_reach_board = self.generate_reachable_tiles_board(piece.color.opposite_color());

        let (can_left_castle, can_right_castle) = match piece.color {
            ChessColor::White => {
                (self.get_white_left_castle(), self.get_white_right_castle())
            },
            ChessColor::Black => {
                (self.get_black_left_castle(), self.get_black_right_castle())
            }
        };

        // Left castle
        if can_left_castle {
            if let Ok(None) = self.get_piece(1, number as i32) {
                if let Ok(None) = self.get_piece(2, number as i32) {
                    if let Ok(None) = self.get_piece(3, number as i32) {
                        if !opponent_reach_board[2][number] && !opponent_reach_board[3][number] && !opponent_reach_board[4][number] {
                            let mut new_board = self.clone();

                            if piece.color == ChessColor::Black {
                                new_board.set_full_moves(new_board.get_full_moves() + 1);
                            }

                            new_board.move_piece(letter as i32, number as i32, 2, number as i32)?;
                            new_board.move_piece(0, number as i32, 3, number as i32)?;
                            new_board.set_en_passant(None);
                            new_board.set_king_pos(piece.color, (2, number));
                            new_board.set_half_moves(self.get_half_moves() + 1);

                            match piece.color {
                                ChessColor::White => {
                                    new_board.set_white_left_castle(false);
                                    new_board.set_white_right_castle(false);
                                },
                                ChessColor::Black => {
                                    new_board.set_black_left_castle(false);
                                    new_board.set_black_right_castle(false);
                                }
                            }

                            let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(3), get_number(number));
                            possible_moves.push((mov_str, new_board));
                        }
                    }
                }
            }
        }

        // Right castle
        if can_right_castle {
            if let Ok(None) = self.get_piece(5, number as i32) {
                if let Ok(None) = self.get_piece(6, number as i32) {
                        if !opponent_reach_board[4][number] && !opponent_reach_board[5][number] && !opponent_reach_board[6][number] {
                            let mut new_board = self.clone();

                            if piece.color == ChessColor::Black {
                                new_board.set_full_moves(new_board.get_full_moves() + 1);
                            }
                            
                            new_board.move_piece(letter as i32, number as i32, 6, number as i32)?;
                            new_board.move_piece(7, number as i32, 5, number as i32)?;
                            new_board.set_en_passant(None);
                            new_board.set_king_pos(piece.color, (6, number));
                            new_board.set_half_moves(self.get_half_moves() + 1);

                            match piece.color {
                                ChessColor::White => {
                                    new_board.set_white_left_castle(false);
                                    new_board.set_white_right_castle(false);
                                },
                                ChessColor::Black => {
                                    new_board.set_black_left_castle(false);
                                    new_board.set_black_right_castle(false);
                                }
                            }

                            let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(6), get_number(number));
                            possible_moves.push((mov_str, new_board));
                        }
                }
            }
        }

        Ok(possible_moves)
    }

    pub fn get_king_pos(&self, color: ChessColor) -> Pos {
        match color {
            ChessColor::White => self.get_white_king_pos(),
            ChessColor::Black => self.get_black_king_pos()
        }
    }

    pub fn set_king_pos(&mut self, color: ChessColor, pos: Pos) {
        match color {
            ChessColor::White => {
                self.set_white_king_pos(pos)
            },
            ChessColor::Black => {
                self.set_black_king_pos(pos)
            }
        }
    }


    // Check if the game has ended for the given player
    pub fn check_for_game_end(&self, turn: ChessColor, board_history_hashmap: &HashMap<NormalBoard, i32>) -> Result<EndType, ChessError> {
        if self.generate_possible_moves(turn)?.is_empty() {
            let reach_board = self.generate_reachable_tiles_board(turn.opposite_color());

            let (letter, number) = self.get_king_pos(turn);
            if reach_board[letter][number] {
                return Ok(EndType::chess_color_to_win_type(turn.opposite_color()));
            } else {
                return Ok(EndType::Tie);
            }
        }

        match self.check_repetition(board_history_hashmap) {
            EndType::NoEnd => (),
            other_res => return Ok(other_res)
        }

        match self.check_half_moves_end() {
            EndType::NoEnd => (),
            other_res => return Ok(other_res)
        }

        Ok(EndType::NoEnd)
    }

    fn check_repetition(&self, board_history_hashmap: &HashMap<NormalBoard, i32>) -> EndType {
        match board_history_hashmap.get(self) {
            Some(occurrences) => {
                if *occurrences >= 3 {
                    EndType::Tie
                } else {
                    EndType::NoEnd
                }
            },
            None => EndType::NoEnd
        }
    }

    fn check_half_moves_end(&self) -> EndType {
        if self.get_half_moves() >= 100 {
            EndType::Tie
        } else {
            EndType::NoEnd
        }
    }
}

impl ChessBoardContract for NormalBoard {
    fn generate_moves(&self, turn: ChessColor, _: &Constants) -> Result<Vec<(String, Self)>, ChessError> {
        self.generate_possible_moves(turn)
    }

    fn check_game_end(&self, turn: ChessColor, board_history: &HashMap<Self, i32>, _: &Constants) -> Result<EndType, ChessError> {
        self.check_for_game_end(turn, board_history)
    }

    fn board_ascii(&self, use_unicode: bool) -> String {
        self.board_ascii(use_unicode)
    }

    fn new_board() -> Self {
        NormalBoard::new_start_board()
    }

    fn get_value_of_pieces(&self, piece_values: [i32; 6]) -> i32 {
        self.iter().map(|(_, piece)| {
            piece_values[piece.typ as usize] * piece.color.side_const()
        }).sum()
    }

    fn board_to_fen(&self, turn: ChessColor) -> String {
        let mut fen = String::new();
    
        let mut none_counter = 0;
        for row in (0..8).rev() {
            for col in 0..8 {
                let piece = self.get_piece(col, row).expect("Normalboard to FEN piece get");
    
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
    
        if !self.get_white_right_castle() && !self.get_white_left_castle() && !self.get_black_right_castle() && !self.get_black_left_castle() {
            fen.push('-');
        } else {
            if self.get_white_right_castle() {
                fen.push('K');
            }
    
            if self.get_white_left_castle() {
                fen.push('Q');
            }
    
            if self.get_black_right_castle() {
                fen.push('k');
            }
    
            if self.get_black_left_castle() {
                fen.push('q');
            }
        }
    
        fen.push(' ');
    
        match self.get_en_passant() {
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
    
        fen.push_str(self.get_half_moves().to_string().as_str());
    
        fen.push(' ');
    
        fen.push_str(self.get_full_moves().to_string().as_str());
    
        fen
    }
}
