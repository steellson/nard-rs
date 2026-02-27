use std::io::stdin;

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

        let mut input = String::new();
        let mut mode: Option<Mode> = None;
        while mode.is_none() {
            input.clear();
            stdin().read_line(&mut input).expect("Input error");
            mode = match input.trim().parse::<u8>() {
                Ok(1) => Some(Mode::Singleplayer),
                Ok(2) => Some(Mode::Multiplayer),
                _ => {
                    println!("Unexpected input, try again");
                    None
                }
            }
        }

        input.clear();
        return mode.unwrap();
    }
}
