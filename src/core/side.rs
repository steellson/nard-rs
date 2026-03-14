pub const SIDES: [&'static str; 2] = ["White chips", "Black chips"];

#[derive(Copy, Clone, PartialEq)]
pub enum Side {
    Black,
    White
}