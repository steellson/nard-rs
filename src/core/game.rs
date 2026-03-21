use super::{
    deck::Deck,
    player::Player, 
    side::Side,
    throw::Throw
};

const PLAYERS: usize = 2;

pub struct Game {
    pub deck: Deck,
    pub step_of: Side,
    pub last_throw: Throw,
    pub players: [Player; PLAYERS]
}

// MARK: - Build game
impl Game {
    pub fn new(players: [Player; PLAYERS]) -> Self {
        // Who step first ...
        // Jackpot isn't available
        let mut throw = Throw::new();
        while throw.is_jackpot { throw = Throw::new(); }

        let deck = Deck::new();
        let dices = &throw.dices;
        
        let is_host_leader = dices[0].result > dices[1].result;
        let host_side = players.iter().find(|p| p.is_host).unwrap().side;
        let step_of = match is_host_leader {
            true => host_side,
            false => host_side.inverted()
        };
        
        Self {
            step_of: step_of,
            deck: deck,
            last_throw: throw,
            players: players,
        }
    }
}

// MARK: - Step
impl Game {
    pub fn step(&mut self) {
        self.step_of = self.step_of.inverted();
    }
    
    pub fn throw(&mut self) {
        let throw = Throw::new();
        self.last_throw = throw;
    }
}