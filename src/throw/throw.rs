use super::dice::Dice;

pub struct Throw {
    pub first: u8,
    pub second: u8,
    pub is_jackpot: bool
}

impl Throw {
    pub fn new() -> Self {
        let first_dice = Dice::roll();
        let second_dice = Dice::roll();
        
        Self {
            first: first_dice,
            second: second_dice,
            is_jackpot: first_dice.eq(&second_dice)
        }
    }
}