use super::side::Side;

#[derive(Copy, Clone)]
pub struct Chip {
    pub side: Side,
    pub selected: bool
}

impl Chip {
    pub fn new(side: Side) -> Self {
        Self { 
            side: side,
            selected: false
        }
    }
}
