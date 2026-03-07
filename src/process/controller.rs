use ratatui::Frame;
use crossterm::event::{KeyCode, KeyEvent};

use crate::ui::border::{self, Border};
use crate::ui::menu::{Menu, NavDirection};
use crate::ui::field::Field;
use crate::core::game::Game;
use crate::core::mode::{Mode, MODES};
use crate::core::side::{Side, SIDES};

enum Scenes {
    SelectMode,
    SelectSide,
    GameDeck
}

pub struct Controller {
    // UI
    menu: Menu,
    field: Field,
    scene: Scenes,
    // Game
    mode: Option<Mode>,
    host_side: Option<Side>,
    game: Option<Game>
}

impl Controller {
    pub fn new() -> Self {        
        Self { 
            menu: Menu::new(MODES),
            field: Field::new(),
            scene: Scenes::SelectMode,
            game: None,
            host_side: None,
            mode: None,
        }
    }
}

// MARK: - Render
impl Controller {
    pub fn render(&mut self, frame: &mut Frame) {
        match self.scene {
            Scenes::SelectMode => {
                Border::render(frame, border::BorderStyle::Menu);
                self.menu.render(frame)
            },
            Scenes::SelectSide => {
                Border::render(frame, border::BorderStyle::Menu);
                self.menu.render(frame)
            },
            Scenes::GameDeck => {
                Border::render(frame, border::BorderStyle::Game);
                self.field.render(frame);
            }
        }
    }
}

// MARK: - Keys handling
impl Controller {
    pub fn handle_key(&mut self, key_event: KeyEvent) {
        match self.scene {
            Scenes::SelectMode => self.handle_menu_keys(key_event),
            Scenes::SelectSide => self.handle_menu_keys(key_event),
            Scenes::GameDeck => self.handle_game_keys(key_event),
        }
    }
    
    fn handle_menu_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => {
                self.menu.select_row(NavDirection::Up);
            },
            KeyCode::Down => {
                self.menu.select_row(NavDirection::Down);
            },
            KeyCode::Enter => {
                match self.scene {
                    Scenes::SelectMode => {
                        self.mode = match self.menu.selected {
                            "Multiplayer" => Some(Mode::Multiplayer),
                            _ => Some(Mode::Singleplayer)
                        };
                        self.menu = Menu::new(SIDES);
                    },
                    Scenes::SelectSide => {
                        self.host_side = match self.menu.selected {
                            "Black" => Some(Side::Black),
                            _ => Some(Side::White)
                        };
                        self.game = Some(Game::new(self.host_side.unwrap()));
                    },
                    Scenes::GameDeck => {
                        // ...
                        // ...
                        // ...
                    }
                }
            }
            _ => {}
        }
    }
    
    fn handle_game_keys(&mut self, key_event: KeyEvent) {
        // ...
        // ...
        // ...
    }
}

// MARK: - Lifecycle
impl Controller {
    fn start(mut self) {
        // Selected mode (... should be taken from UI)
        self.mode = Some(Mode::Singleplayer);
        // Host select side (... should be taken from UI)
        self.host_side = Some(Side::White);
        // Init game
        self.game = Some(Game::new(self.host_side.unwrap()));
    }
       
    fn process(self) {
        // Steps (from UI)
        // ... Need eceive coordinates from users with UI model
        self.game.unwrap().step();
    }
    
    fn end(mut self) {
        self.game = None;
        self.mode = None;
        self.host_side = None;
        // ... Navigate go start menu screen
    }
}