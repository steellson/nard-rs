use super::chip::Chip;
use super::deck::SIDE_CHIPS;

#[derive(Clone, Copy)]
pub struct Row {
    num: u8,
    chips: Option<[Chip; SIDE_CHIPS]>
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