use super::player::Player;
use super::deck::Deck;

const PLAYERS: usize = 2;

#[derive(Debug)]
pub struct Game {
    pub deck: Deck,
    pub players: [Player; PLAYERS]
}

impl Game {
    pub fn new(players: [Player; PLAYERS]) -> Self { 
        Self {
            deck: Deck::new(),
            players: players
        }
    }
}