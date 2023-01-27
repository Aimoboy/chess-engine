
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChessColor {
    White = 0,
    Black = 1
}

impl ChessColor {
    pub fn opposite_color(&self) -> ChessColor {
        match self {
            ChessColor::White => ChessColor::Black,
            ChessColor::Black => ChessColor::White
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            ChessColor::White => "White".to_string(),
            ChessColor::Black => "Black".to_string()
        }
    }

    pub fn side_const(&self) -> i32 {
        match self {
            ChessColor::White => 1,
            ChessColor::Black => -1
        }
    }
}
