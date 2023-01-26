use crate::enums::{
    piece_color::PieceColor,
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

#[derive(Debug, Clone, PartialEq)]
pub struct ChessPiece {
    pub typ: PieceType,
    pub color: PieceColor,
    moved: bool
}

impl ChessPiece {
    pub fn new(typ: PieceType, color: PieceColor) -> Self {
        Self {
            typ,
            color,
            moved: false
        }
    }

    // Maybe this and the next function should just use self?
    fn piece_to_char(piece: &ChessPiece) -> char {
        match piece.color {
            PieceColor::White => match piece.typ {
                                    PieceType::Pawn => 'p',
                                    PieceType::Rook => 'r',
                                    PieceType::Knight => 'n',
                                    PieceType::Bishop => 'b',
                                    PieceType::Queen => 'q',
                                    PieceType::King => 'k'
                                }
            PieceColor::Black => match piece.typ {
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
            PieceColor::White => match piece.typ {
                                    PieceType::Pawn => '\u{265F}',
                                    PieceType::Rook => '\u{265C}',
                                    PieceType::Knight => '\u{265E}',
                                    PieceType::Bishop => '\u{265D}',
                                    PieceType::Queen => '\u{265B}',
                                    PieceType::King => '\u{265A}'
                                }
            PieceColor::Black => match piece.typ {
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
    en_passent: Option<(i32, i32)>,
    white_left_castle: bool,
    white_right_castle: bool,
    black_left_castle: bool,
    black_right_castle: bool,
    half_moves_since_piece_capture_or_pawn_advance: i32,
    full_move_counter: i32
}

impl NormalBoard {
    pub fn set_en_passent(&mut self, val: Option<(i32, i32)>) {
        self.en_passent = val;
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

    pub fn get_en_passent(&self) -> Option<(i32, i32)> {
        return self.en_passent;
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
                let mut piece = piece.clone();
                piece.moved = true;
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
            en_passent: None,
            white_left_castle: false,
            white_right_castle: false,
            black_left_castle: false,
            black_right_castle: false,
            half_moves_since_piece_capture_or_pawn_advance: 0,
            full_move_counter: 1
        }
    }

    pub fn new_start_board() -> Self {
        let mut board = Self::blank_board();

        board[0][0] = Some(ChessPiece::new(PieceType::Rook, PieceColor::White));
        board[1][0] = Some(ChessPiece::new(PieceType::Knight, PieceColor::White));
        board[2][0] = Some(ChessPiece::new(PieceType::Bishop, PieceColor::White));
        board[3][0] = Some(ChessPiece::new(PieceType::Queen, PieceColor::White));
        board[4][0] = Some(ChessPiece::new(PieceType::King, PieceColor::White));
        board[5][0] = Some(ChessPiece::new(PieceType::Bishop, PieceColor::White));
        board[6][0] = Some(ChessPiece::new(PieceType::Knight, PieceColor::White));
        board[7][0] = Some(ChessPiece::new(PieceType::Rook, PieceColor::White));

        for i in 0..8 {
            board[i][1] = Some(ChessPiece::new(PieceType::Pawn, PieceColor::White));
        }

        for i in 0..8 {
            board[i][6] = Some(ChessPiece::new(PieceType::Pawn, PieceColor::Black));
        }

        board[0][7] = Some(ChessPiece::new(PieceType::Rook, PieceColor::Black));
        board[1][7] = Some(ChessPiece::new(PieceType::Knight, PieceColor::Black));
        board[2][7] = Some(ChessPiece::new(PieceType::Bishop, PieceColor::Black));
        board[3][7] = Some(ChessPiece::new(PieceType::Queen, PieceColor::Black));
        board[4][7] = Some(ChessPiece::new(PieceType::King, PieceColor::Black));
        board[5][7] = Some(ChessPiece::new(PieceType::Bishop, PieceColor::Black));
        board[6][7] = Some(ChessPiece::new(PieceType::Knight, PieceColor::Black));
        board[7][7] = Some(ChessPiece::new(PieceType::Rook, PieceColor::Black));

        Self {
            board: board,
            en_passent: None,
            white_left_castle: true,
            white_right_castle: true,
            black_left_castle: true,
            black_right_castle: true,
            half_moves_since_piece_capture_or_pawn_advance: 0,
            full_move_counter: 1
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
                        if piece.typ != PieceType::Pawn || piece.color != PieceColor::White {
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

    pub fn generate_reachable_tiles_board(&self, color: PieceColor) -> ReachBoard {
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

    pub fn generate_possible_moves(&self, prev_board: Option<&NormalBoard>, turn: PieceColor) -> Result<Vec<(String, NormalBoard)>, ChessError> {
        let res = self.iter()
            .filter(|(_, piece)| piece.color == turn)
            .map(|(pos, piece)| self.generate_possible_moves_for_piece(piece, prev_board, turn, pos))
            .fold(Ok(Vec::with_capacity(50)), |acc, item| {
                let mut acc_val: Vec<(String, NormalBoard)> = acc?;
                let mut item_val = item?;
                acc_val.append(&mut item_val);

                Ok(acc_val)
            });
        
        Ok(res?)
    }

    fn generate_possible_moves_for_piece(&self, piece: &ChessPiece, prev_board: Option<&NormalBoard>, turn: PieceColor, pos: Pos) -> Result<Vec<(String, NormalBoard)>, ChessError> {
        Ok(match piece.typ {
            PieceType::Pawn => self.generate_possible_pawn_moves(piece, prev_board, pos),
            PieceType::Rook => self.generate_possible_rook_moves(piece, pos),
            PieceType::Knight => self.generate_possible_knight_moves(piece, pos),
            PieceType::Bishop => self.generate_possible_bishop_moves(piece, pos),
            PieceType::Queen => self.generate_possible_queen_moves(piece, pos),
            PieceType::King => self.generate_possible_king_moves(piece, pos)
        }?.into_iter().filter(|(_, board)| Self::check_if_valid_move(board, turn)).collect())
    }

    fn check_if_valid_move(board: &NormalBoard, turn: PieceColor) -> bool {
        let opponent_reach_board = board.generate_reachable_tiles_board(turn.opposite_color());

        if let Ok((king_pos_letter, king_pos_number)) = board.get_king_pos(turn) {
            return !opponent_reach_board[king_pos_letter][king_pos_number];
        }

        false
    }

    fn generate_possible_pawn_moves(&self, piece: &ChessPiece, prev_board: Option<&NormalBoard>, pos: Pos) -> Result<Vec<(String, NormalBoard)>, ChessError> {
        let (letter, number) = pos;
        let side_const = piece.color.side_const();

        let mut possible_moves = Vec::new();

        // One step
        let new_letter = letter as i32;
        let new_number = number as i32 + side_const;
        if let Ok(None) = self.get_piece(letter as i32, new_number) {
            let mut new_board = self.clone();
            new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;

            let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
            possible_moves.push((mov_str, new_board));
        }

        // Double step
        if !piece.moved {
            let new_letter = letter as i32;
            let new_number = number as i32 + side_const;
            if let Ok(None) = self.get_piece(letter as i32, new_number) {
                let new_number = number as i32 + side_const * 2;
                if let Ok(None) = self.get_piece(letter as i32, new_number) {
                    let mut new_board = self.clone();
                    new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;

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
                    let mut new_board = self.clone();
                    new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;

                    let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                    possible_moves.push((mov_str, new_board));
                }
            }
        }

        // En passant
        for dir in direction {
            let new_letter = letter as i32 + dir;
            let new_number = number as i32;
            if let Ok(Some(other_piece)) = self.get_piece(new_letter, new_number) {
                if other_piece.color != piece.color && other_piece.typ == PieceType::Pawn {
                    let new_letter = letter as i32 + dir;
                    let new_number = number as i32 + side_const;
                    if let Ok(None) = self.get_piece(new_letter, new_number) {
                        let new_letter = letter as i32 + dir;
                        let new_number = number as i32 + side_const * 2;
                        if let Ok(None) = self.get_piece(new_letter, new_number) {
                            if let Some(prev_board) = prev_board {
                                if let Ok(Some(prev_piece)) = prev_board.get_piece(new_letter, new_number) {
                                    if prev_piece.color != piece.color && prev_piece.typ == PieceType::Pawn {
                                        let mut new_board = self.clone();
                                        new_board.move_piece(letter as i32, number as i32, letter as i32 + dir, number as i32 + side_const)?;
                                        new_board.delete_piece(letter as i32 + dir, number as i32)?;
                                        
                                        let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter((letter as i32 + dir) as usize), get_number((number as i32 + side_const) as usize));
                                        possible_moves.push((mov_str, new_board));
                                    }
                                }
                            }
                        }
                    }
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
                        new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;

                        let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                        possible_moves.push((mov_str, new_board));
                    },
                    Ok(Some(other_piece)) => {
                        if piece.color != other_piece.color {
                            let mut new_board = self.clone();
                            new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;

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
                    new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;

                    let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                    possible_moves.push((mov_str, new_board));
                },
                Ok(Some(other_piece)) => {
                    if piece.color != other_piece.color {
                        let mut new_board = self.clone();
                        new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;

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
                        new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;

                        let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                        possible_moves.push((mov_str, new_board));
                    },
                    Ok(Some(other_piece)) => {
                        if piece.color != other_piece.color {
                            let mut new_board = self.clone();
                            new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;
    
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
                new_board.move_piece(letter as i32, number as i32, new_letter, new_number)?;

                let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(new_letter as usize), get_number(new_number as usize));
                possible_moves.push((mov_str, new_board));
            }
        }

        let opponent_reach_board = self.generate_reachable_tiles_board(piece.color.opposite_color());

        // Left castle
        if !piece.moved {
            if let Ok(Some(rook_piece)) = self.get_piece(0, number as i32) {
                if rook_piece.typ == PieceType::Rook && !rook_piece.moved {
                    if let Ok(None) = self.get_piece(1, number as i32) {
                        if let Ok(None) = self.get_piece(2, number as i32) {
                            if let Ok(None) = self.get_piece(3, number as i32) {
                                if !opponent_reach_board[2][number] && !opponent_reach_board[3][number] && !opponent_reach_board[4][number] {
                                    let mut new_board = self.clone();
                                    new_board.move_piece(letter as i32, number as i32, 2, number as i32)?;
                                    new_board.move_piece(0, number as i32, 3, number as i32)?;

                                    let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(3), get_number(number));
                                    possible_moves.push((mov_str, new_board));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Right castle
        if !piece.moved {
            if let Ok(Some(rook_piece)) = self.get_piece(7, number as i32) {
                if rook_piece.typ == PieceType::Rook && !rook_piece.moved {
                    if let Ok(None) = self.get_piece(5, number as i32) {
                        if let Ok(None) = self.get_piece(6, number as i32) {
                            if !opponent_reach_board[4][number] && !opponent_reach_board[5][number] && !opponent_reach_board[6][number] {
                                let mut new_board = self.clone();
                                new_board.move_piece(letter as i32, number as i32, 6, number as i32)?;
                                new_board.move_piece(7, number as i32, 5, number as i32)?;

                                let mov_str = format!("{}{} {}{}", get_letter(letter), get_number(number), get_letter(6), get_number(number));
                                possible_moves.push((mov_str, new_board));
                            }
                        }
                    }
                }
            }
        }

        Ok(possible_moves)
    }

    pub fn get_king_pos(&self, color: PieceColor) -> Result<Pos, ChessError> {
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = &self.board[i][j] {
                    if piece.typ == PieceType::King && piece.color == color {
                        return Ok((i, j));
                    }
                }
            }
        }

        Err(ChessError::NoKing)
    }


    // Check if the game has ended for the given player
    pub fn check_for_game_end(&self, prev_board: Option<&NormalBoard>, turn: PieceColor) -> Result<EndType, ChessError> {
        if self.generate_possible_moves(prev_board, turn)?.len() == 0 {
            let reach_board = self.generate_reachable_tiles_board(turn.opposite_color());

            if let Ok((letter, number)) = self.get_king_pos(turn) {
                if reach_board[letter][number] {
                    return Ok(EndType::Checkmate(turn.opposite_color()));
                } else {
                    return Ok(EndType::Tie);
                }
            }
        }

        Ok(EndType::NoEnd)
    }

    pub fn check_repetition(&self, board_history: &Vec<NormalBoard>) -> EndType {
        let mut count = 0;

        for b in board_history {
            if self.check_board_equality(b) {
                count += 1;
            }
        }

        if count >= 3 {
            return EndType::Tie;
        }

        EndType::NoEnd
    }

    fn check_board_equality(&self, other: &NormalBoard) -> bool {
        for i in 0..8 {
            for j in 0..8 {
                if self.board[i][j].is_none() && !other.board[i][j].is_none() {
                    return false;
                }

                if let Some(piece) = &self.board[i][j] {
                    if let Some(other_piece) = &other.board[i][j] {
                        if piece.typ != other_piece.typ || piece.color != other_piece.color {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl ChessBoardContract for NormalBoard {
    fn generate_moves(&self, prev_board: Option<&Self>, turn: PieceColor, _: &Constants) -> Result<Vec<(String, Self)>, ChessError> {
        self.generate_possible_moves(prev_board, turn)
    }

    fn check_game_end(&self, prev_board: Option<&Self>, turn: PieceColor, _: &Constants) -> Result<EndType, ChessError> {
        self.check_for_game_end(prev_board, turn)
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
}
