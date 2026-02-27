use std::io::stdin;

pub struct Inputer {}
impl Inputer {
    pub fn select_from_stdin(variants: u8) -> u8 {
        let mut input = String::new();
        let mut res: Option<u8> = None;
        
        while res.is_none() {
            input.clear();
            stdin().read_line(&mut input).expect("Input error");
            res = match input.trim().parse::<u8>() {
                Ok(n) => {
                    if (1..=variants).contains(&n) {
                        Some(n)
                    } else {
                        println!("Expected values from 1 to {}", variants);
                        None
                    }
                },
                Err(_) => {
                    println!("Unexpected input, try again");
                    None
                }
            }
        }
        
        input.clear();
        return res.unwrap();
    }
}