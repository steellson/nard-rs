use super::chip::Chip;
use super::deck::SIDE_CHIPS;

#[derive(Clone, Copy, Debug)]
pub struct Row {
    pub num: u8,
    pub chips: Option<[Chip; SIDE_CHIPS]>
}

impl Row {
    pub fn new(
        num: u8, 
        chips: Option<[Chip; SIDE_CHIPS]>
    ) -> Self {
        Self { 
            num: num,
            chips 
        }
    }
}