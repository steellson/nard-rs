pub const MODES: [&'static str; 2] = ["Singleplayer", "Multuplayer"];

#[derive(PartialEq)]
pub enum Mode {
    Singleplayer,
    Multiplayer,
}