use super::{
    deck::Deck,
    throw::Throw,
    player::Player,
    step::{Step, NavDirection}
};

const PLAYERS: usize = 2;

pub struct Game {
    pub deck: Deck,
    pub step: Step,
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
        let step_of_side = match is_host_leader {
            true => host_side,
            false => host_side.inverted()
        };
        let initial_step = Step::new(
            step_of_side, 
            throw,
            None,
            None,
            0
        );
        
        Self {
            deck: deck,
            step: initial_step,
            players: players,
        }
    }
}

// MARK: - Methods
impl Game {
    pub fn make_throw(&mut self) {
        self.step.throw = Throw::new();
    }

    pub fn select_row(&mut self, direction: NavDirection) {
        match direction {
            NavDirection::Up => {
                todo!();
            },
            NavDirection::Down => {
                todo!();
            }
        }
    }

    pub fn select_chip(&mut self, direction: NavDirection) {  
        match direction {
            NavDirection::Up => {
                todo!();
            },
            NavDirection::Down => {
                todo!();
            }
        }
    }

    pub fn apply_step(&mut self) {
        // ... ???
        self.step = Step::new(
            self.step.side.inverted(),
            self.step.throw,
            None,
            None,
            4
        );
    }
}