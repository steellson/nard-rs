use super::chip::Chip;
use super::deck::SIDE_CHIPS;
use super::row::Row;

const ROWS_IN_SECTOR: usize = 6;

#[derive(PartialEq)]
pub enum Placement {
    A, B, C, D
}

pub struct Sector {
    placement: Placement,
    rows: [Row; ROWS_IN_SECTOR]
}

impl Sector {
    pub fn new(
        placement: Placement,
        chips: Option<[Chip; SIDE_CHIPS]>
    ) -> Self {
        let rows: [Row; ROWS_IN_SECTOR] = std::array::from_fn(|i| 
            Row::new((i + 1) as u8, chips)
        );
        
        Self {
            placement,
            rows: rows,
        }
    }
}