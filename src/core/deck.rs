use super::sector::{Sector, Placement};
use super::player::Side;
use super::chip::Chip;

pub const SIDE_CHIPS: usize = 15;
const ALL_CHIPS: usize = 30;
const SECTORS: usize = 4;

#[derive(Debug)]
pub struct Deck {
    pub sectors: [Sector; SECTORS]
}

impl Deck {
    pub fn new() -> Self {
        let white = [Chip::new(Side::White); SIDE_CHIPS];
        let black = [Chip::new(Side::Black);  SIDE_CHIPS];
        
        let mut chips = [Chip::new(Side::Black); ALL_CHIPS];
        chips[..SIDE_CHIPS].copy_from_slice(&white);
        chips[SIDE_CHIPS..].copy_from_slice(&black);
        
        Self {
            sectors: [
                Sector::new(Placement::A, Some(white)),
                Sector::new(Placement::B, None),
                Sector::new(Placement::C, Some(black)),
                Sector::new(Placement::D, None),
            ]
        }
    }
}