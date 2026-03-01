use super::{
    deck::Deck,
    player::Player, 
    side::Side,
    throw::Throw
};

const PLAYERS: usize = 2;

#[derive(Debug)]
pub struct Game {
    step_of: Side,
    deck: Deck,
    last_throw: Throw,
    players: [Player; PLAYERS]
}

// MARK: - Build game
impl Game {
    pub fn new(host_side: Side) -> Self {
        let opposite_side = match host_side {
            Side::White => Side::Black,
            Side::Black => Side::White
        };
        
        // Who step first ...
        // Jackpot isn't available
        let mut throw = Throw::new();
        while throw.is_jackpot { throw = Throw::new(); }
        
        let dices = &throw.dices;
        let is_host_leader = dices[0].result > dices[1].result;
        let fist_step_of = match is_host_leader {
            true => host_side,
            false => opposite_side
        };
        
        Self {
            step_of: fist_step_of,
            deck: Deck::new(),
            last_throw: throw,
            players: [
                Player::new(host_side),
                Player::new(opposite_side)
            ],
        }
    }
}

// MARK: - Step
impl Game {
    pub fn step(&mut self) {
        self.step_of = match self.step_of {
            Side::White => Side::Black,
            Side::Black => Side::White
        };
        let throw = Throw::new();
        println!("Step");
    }
}