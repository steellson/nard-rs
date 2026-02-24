use super::player::{Player, Side};
use super::deck::Deck;

const PLAYERS: usize = 2;

#[derive(Debug)]
pub struct Game {
    pub deck: Deck,
    pub players: [Player; PLAYERS]
}

impl Game {
    pub fn new() -> Self { 
        Self {
            deck: Deck::new(),
            players: [
                Player::new(Side::White),
                Player::new(Side::Black)
            ]
        }
    }
}