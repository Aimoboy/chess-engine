use serde::Serialize;

use crate::enums::chess_color::ChessColor;

#[derive(PartialEq, Clone, Copy, Debug, Serialize)]
pub enum EndType {
    NoEnd,
    Tie,
    WhiteWin,
    BlackWin
}

impl EndType {
    pub fn chess_color_to_win_type(color: ChessColor) -> EndType {
        match color {
            ChessColor::White => EndType::WhiteWin,
            ChessColor::Black => EndType::BlackWin
        }
    }
}
