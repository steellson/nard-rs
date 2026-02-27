use std::io::stdin;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    Black, White
}

impl Side {
    pub fn from_input() -> Self {
        println!("Select host player side:");
        println!("White - 1");
        println!("Black - 2");

        let mut input = String::new();
        let mut side: Option<Side> = None;
        while side.is_none() {
            input.clear();
            stdin().read_line(&mut input).expect("Input error");
            side = match input.trim().parse::<u8>() {
                Ok(1) => Some(Side::White),
                Ok(2) => Some(Side::Black),
                _ => {
                    println!("Unexpected input, try again");
                    None
                }
            }
        }

        input.clear();
        return side.unwrap();
    }
}