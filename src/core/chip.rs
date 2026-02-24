use super::player::Side;

#[derive(Copy, Clone, Debug)]
pub struct Chip {
    pub side: Side
}

impl Chip {
    pub fn new(side: Side) -> Self {
        Self { side }
    }
}