use super::chip::Chip;
use super::deck::SIDE_CHIPS;
use super::row::Row;

const ROWS_IN_SECTOR: usize = 6;

#[derive(Debug, PartialEq)]
pub enum Placement {
    A, B, C, D
}

#[derive(Debug)]
pub struct Sector {
    pub placement: Placement,
    pub rows: [Row; ROWS_IN_SECTOR]
}

impl Sector {
    pub fn new(
        placement: Placement,
        chips: Option<[Chip; SIDE_CHIPS]>
    ) -> Self {
        let rows: [Row; ROWS_IN_SECTOR] = std::array::from_fn(|i| {
            let num = (i + 1) as u8;
            let chips = if num == 1 { chips } else { None };
            Row::new(num, chips)
        });
        
        Self {
            placement,
            rows: rows,
        }
    }
}