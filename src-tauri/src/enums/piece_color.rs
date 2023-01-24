
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceColor {
    White = 0,
    Black = 1
}

impl PieceColor {
    pub fn opposite_color(&self) -> PieceColor {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            PieceColor::White => "White".to_string(),
            PieceColor::Black => "Black".to_string()
        }
    }

    pub fn side_const(&self) -> i32 {
        match self {
            PieceColor::White => 1,
            PieceColor::Black => -1
        }
    }
}
