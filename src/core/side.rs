use crate::tools::inputer::Inputer;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    Black, White
}

impl Side {
    pub fn from_input() -> Self {
        println!("Select host player side:");
        println!("White - 1");
        println!("Black - 2");

        match Inputer::select_from_stdin(2) {
            1 => Side::White,
            2 => Side::Black,
            _ => {
                println!("Side selection warning!");
                Side::White
            }
        }
    }
}