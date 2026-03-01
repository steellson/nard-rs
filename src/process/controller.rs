use crate::ui::menu::{MenuSelector, MenuBorder};
use crate::core::{game::Game, side::Side, mode::Mode};

#[derive(Debug)]
pub struct Controller {
    // UI
    pub menu_botder: MenuBorder,
    pub menu: MenuSelector,
    // Game
    mode: Option<Mode>,
    host_side: Option<Side>,
    game: Option<Game>
}

impl Controller {
    pub fn new() -> Self {
        Self { 
            menu_botder: MenuBorder::new(),
            menu: MenuSelector::new(),
            game: None,
            host_side: None,
            mode: None,
        }
    }
}

// MARK: - Lifecycle
impl Controller {
    pub fn start(mut self) {
        // Selected mode (... should be taken from UI)
        self.mode = Some(Mode::Singleplayer);
        // Host select side (... should be taken from UI)
        self.host_side = Some(Side::White);
        // Init game
        self.game = Some(Game::new(self.host_side.unwrap()));
    }
    
    pub fn process(mut self) {
        // Steps (from UI)
        // ... Need eceive coordinates from users with UI model
        self.game.unwrap().step();
    }
    
    pub fn end(mut self) {
        self.game = None;
        self.mode = None;
        self.host_side = None;
        // ... Navigate go start menu screen
    }
}