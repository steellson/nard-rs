use super::dice::Dice;

const DICES: usize = 2;

#[derive(Debug)]
pub struct Throw {
    pub dices: [Dice; DICES],
    pub is_jackpot: bool
}

impl Throw {
    pub fn new() -> Self {
        let dices = [Dice::new(), Dice::new()];
        let is_jackpot = dices[0].eq(&dices[1]);
        
        Self {
            dices: dices,
            is_jackpot: is_jackpot
        }
    }
}