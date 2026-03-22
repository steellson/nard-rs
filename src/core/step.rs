use super::{
    row::Row,
    chip::Chip,
    side::Side,
    throw::Throw
};

const MAX_CHIPS_FOR_STEP: usize = 4;

pub enum NavDirection {
    Up, Down
}

pub struct Step {
    pub side: Side,
    pub throw: Throw,
    pub row: Option<Row>,
    pub chips: Option<[Chip; MAX_CHIPS_FOR_STEP]>,
    pub possible_ways: u8
}

impl Step {
    pub fn new(
        side: Side, 
        throw: Throw,
        row: Option<Row>,
        chips: Option<[Chip; MAX_CHIPS_FOR_STEP]>,
        possible_ways: u8
    ) -> Self {
        Self {
            side: side,
            throw: throw,
            row: row,
            chips: chips,
            possible_ways: possible_ways
        }
    }
}