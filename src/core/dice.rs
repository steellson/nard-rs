use rand;

const MAX_VAL: u8 = 6;

#[derive(Debug, PartialEq)]
pub struct Dice {
    pub result: u8
}

impl Dice {
    pub fn new() -> Self {
        Self {
            result: rand::random_range(1..=MAX_VAL)
        }
    }
}