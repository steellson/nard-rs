use super::sector::Placement;
use super::sides::Side;

pub struct Player {
    pub side: Side,
    pub final_sector: Placement,
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
            final_sector: final_sector,
        }
    }
}
