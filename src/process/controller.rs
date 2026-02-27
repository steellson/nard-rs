use crate::core::{mode::Mode, player::Player, side::Side, game::Game, throw::Throw};

pub struct Controller {}

impl Controller {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn start(self) {
        // Ask mode
        let mode = Mode::new();
        print!("Selected mode: {:?}\n", mode);

        // Side selection
        let host_side = Side::from_input();
        let host_player = Player::new(host_side);
        print!("Host select {:?} side\n", host_player);
        
        let opposite_side = if host_side == Side::Black {
            Side::White
        } else {
            Side::Black
        };
        let opposite_player = Player::new(opposite_side);
        let players = [host_player, opposite_player];
            
        // Start
        let game = Game::new(players);
        println!("Game initizlized!\n{:#?}\n", game);
        
        // Initial throw
        let throw = Throw::new();
        println!(
            "Throw!\nDices: {:#?}, jackpot? - {:#?}\n",
            throw.dices, throw.is_jackpot
        );
        
        // Bye bye
        println!("Exit!");
    }
}