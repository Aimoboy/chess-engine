use crate::enums::piece_color::PieceColor;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum EndType {
    NoEnd,
    Tie,
    Checkmate(PieceColor)
}
