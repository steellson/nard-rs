mod core;

use crate::core::{game::Game, throw::Throw};

fn main() {
    debug_log();
}

/* ----- DEBUG ----- */
fn debug_log() {
    let game = Game::new();
    println!("Game initizlized!\n{:#?}\n", game);

    let throw = Throw::new();
    println!(
        "Throw!\nDices: {:#?}, jackpot? - {:#?}\n",
        throw.dices, throw.is_jackpot
    );
}
