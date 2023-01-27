use crate::enums::chess_color::ChessColor;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum EndType {
    NoEnd,
    Tie,
    Checkmate(ChessColor)
}
