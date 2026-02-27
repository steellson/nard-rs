use crate::tools::inputer::Inputer;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Singleplayer,
    Multiplayer,
}

impl Mode {
    pub fn new() -> Self {
        println!("Select mode:");
        println!("Singleplayer - 1");
        println!("Multyplayer  - 2");

        match Inputer::select_from_stdin(2) {
            1 => Mode::Singleplayer,
            2 => Mode::Multiplayer,
            _ => {
                println!("Mode selection warning!");
                Mode::Singleplayer
            }
        }
    }
}
