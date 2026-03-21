pub const ALL_MODES: [Mode; 2] = [Mode::Singleplayer, Mode::Multiplayer];

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Singleplayer,
    Multiplayer,
}

impl Mode {
    pub fn raw_value(self) -> &'static str {
        match self {
            Self::Singleplayer => "Singleplayer",
            Self::Multiplayer => "Multuplayer"
        }
    }
    
    pub fn inverted(&self) -> Mode {
        match self {
            Mode::Singleplayer => Mode::Multiplayer,
            Mode::Multiplayer => Mode::Singleplayer
        } 
    }
}