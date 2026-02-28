use crate::core::{game::Game, mode::Mode, player::Player, side::Side, step::Step, throw::Throw};

pub struct Controller {}

impl Controller {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(self) {
        // Prepare game
        let game = self.prepare_game();
        println!("Game started!\nFirst step: {:#?}\n", game.leading_side);

        // First mock steps of first player
        // One chip from head, one chip doesnt
        let throw = Throw::new(false);
        game.next_step(Step::new(&throw, true, false));
        game.next_step(Step::new(&throw, false, false));

        // Bye bye
        println!("Exit!");
    }
}

impl Controller {
    fn prepare_game(self) -> Game {
        // Ask mode
        let mode = Mode::new();
        print!("Selected mode: {:?}\n", mode);

        // Side selection
        let host_side = Side::from_input();
        let host_player = Player::new(host_side, true);
        print!("Host select {:?} side\n", host_player);

        let opposite_side = if host_side == Side::Black {
            Side::White
        } else {
            Side::Black
        };
        let opposite_player = Player::new(opposite_side, false);
        let players = [host_player, opposite_player];

        // Initial throw
        let throw = Throw::new(true);
        println!("Throw!\nDices: {:?}\n", throw.dices);

        // Building game
        let is_host_leader = throw.dices[0].result > throw.dices[1].result;
        let leading_side = if is_host_leader {
            host_side
        } else {
            opposite_side
        };

        Game::new(players, leading_side)
    }
}
