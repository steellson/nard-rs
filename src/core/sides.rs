const SIDES: usize = 2;

#[derive(Copy, Clone, PartialEq)]
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
}

pub struct Sides {
    pub selected: usize,
    pub source: [Side; SIDES]
}

impl Sides {
    pub fn new() -> Self {
        Self { 
            selected: 0,
            source: [
                Side::White,
                Side::Black
            ]
        }
    }
    
    pub fn toggle_selected(&mut self) {
        self.selected = match self.selected { 
            0 => 1, _ => 0
        };
    }
}