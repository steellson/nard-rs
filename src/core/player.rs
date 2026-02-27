use super::sector::Placement;
use super::side::Side;

#[derive(Debug)]
pub struct Player {
    pub side: Side,
    pub final_sector: Placement
}

impl Player {
    pub fn new(side: Side) -> Self {
        let final_sector = if side == Side::White {
            Placement::D
        } else {
            Placement::B 
        };
        
        Self { 
            side: side,
            final_sector: final_sector
        } 
    }
}