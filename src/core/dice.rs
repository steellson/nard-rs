use rand;

const MAX_VAL: u8 = 6;

pub struct Dice {}

impl Dice {
    pub fn roll() -> u8 {
        rand::random_range(1..=MAX_VAL)
    }
}