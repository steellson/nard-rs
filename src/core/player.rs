use super::sector::Placement;
use super::side::Side;

pub struct Player {
    pub side: Side,
    pub final_sector: Placement,
    pub is_host: bool
}

impl Player {
    pub fn new(is_host: bool, side: Side) -> Self {
        let final_sector = if side == Side::White {
            Placement::D
        } else {
            Placement::B
        };

        Self {
            side: side,
            final_sector: final_sector,
            is_host: is_host
        }
    }
}
