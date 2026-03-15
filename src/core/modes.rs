const MODES: usize = 2;

#[derive(Copy, Clone, PartialEq)]
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
}

pub struct Modes {
    pub selected: usize,
    pub source: [Mode; MODES]
}

impl Modes {
    pub fn new() -> Self {
        Self {
            selected: 0,
            source: [
                Mode::Singleplayer,
                Mode::Multiplayer
            ]
        }
    }
    
    pub fn toggle_selected(&mut self) {
        self.selected = match self.selected { 
            0 => 1, _ => 0
        };
    }
}