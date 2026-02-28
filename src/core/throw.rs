use super::dice::Dice;

pub const DICES: usize = 2;

#[derive(Debug)]
pub struct Throw {
    pub dices: [Dice; DICES],
    pub is_jackpot: bool
}

impl Throw {
    pub fn new(is_initial: bool) -> Self {
        let mut dices = [Dice::new(), Dice::new()];
        let mut is_jackpot = dices[0].eq(&dices[1]);
        
        if is_initial {
            while is_jackpot {
                dices = [Dice::new(), Dice::new()];
                is_jackpot = dices[0].eq(&dices[1]);
            }
        }
        
        Self {
            dices: dices,
            is_jackpot: is_jackpot
        }
    }
}