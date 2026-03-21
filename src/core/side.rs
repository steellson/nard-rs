pub const ALL_SIDES: [Side; 2] = [Side::White, Side::Black];

#[derive(Clone, Copy, PartialEq)]
pub enum Side {
    White,
    Black
}

impl Side {
    pub fn raw_value(self) -> &'static str {
        match self {
            Self::White => "White",
            Self::Black => "Black"
        }
    }
    
    pub fn inverted(&self) -> Side {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White
        } 
    }
}