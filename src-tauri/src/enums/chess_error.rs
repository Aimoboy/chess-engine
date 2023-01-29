
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ChessError {
    InvalidMove = 0,
    NoMovesFound = 1,
    OutsideBounds = 2,
    InvalidMoveString = 3,
    EndWithNoEnd = 4
}
