use super::player::Side;

#[derive(Copy, Clone)]
pub struct Chip {
    side: Side
}

impl Chip {
    pub fn new(side: Side) -> Self {
        Self { side }
    }
}