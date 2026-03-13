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
    scene: Scenes,
    // Game
    mode: Option<Mode>,
    host_side: Option<Side>,
    game: Option<Game>
}

impl Controller {
    pub fn new() -> Self {        
        Self { 
            menu: Menu::new("SELECT GAME MODE:", MODES),
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
                match &mut self.game {
                    Some(g) => Field::new().render(frame, &g.deck),
                    None => {}
                }
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
                        self.scene = Scenes::SelectSide;
                        self.menu = Menu::new("SELECT YOUR SIDE:", SIDES);
                    },
                    Scenes::SelectSide => {
                        self.host_side = match self.menu.selected {
                            "Black" => Some(Side::Black),
                            _ => Some(Side::White)
                        };
                        self.game = Some(Game::new(self.host_side.unwrap()));
                        self.scene = Scenes::GameDeck;
                    },
                    _ => {}
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