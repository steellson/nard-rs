use super::deck::Deck;
use super::player::Player;
use super::side::Side;
use super::step::Step;

const PLAYERS: usize = 2;

#[derive(Debug)]
pub struct Game {
    pub deck: Deck,
    pub players: [Player; PLAYERS],
    pub leading_side: Side,
}

impl Game {
    pub fn new(
        players: [Player; PLAYERS], 
        leading_side: Side
    ) -> Self {
        Self {
            deck: Deck::new(),
            players: players,
            leading_side: leading_side,
        }
    }

    pub fn next_step(&self, step: Step) {
        let from_head = if step.is_from_head {
            "from head"
        } else {
            "not from head"
        };
        
        println!(
            "Step: {:?} / {:?} / {}",
            step.throw.dices[0],
            step.throw.dices[1], 
            from_head
        );
    }
}
