
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ChessError {
    InvalidMove = 0,
    NoMovesFound = 1,
    NoKing = 2,
    OutsideBounds = 3,
    InvalidMoveString = 4,
    EndWithNoEnd = 5
}
