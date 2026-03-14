use ratatui::Frame;
use crossterm::event::{KeyCode, KeyEvent};

use crate::ui::{
    field::Field, 
    popup::Popup,
    menu::{Menu, NavDirection},
    border::{Border, BorderStyle},
};
use crate::core::{
    game::Game,
    mode::{Mode, MODES},
    side::{Side, SIDES}
};

enum Scenes {
    GameMenu,
    GameDeck
}

pub struct Controller {
    // UI
    menu: Menu,
    scene: Scenes,
    // Game
    mode: Option<Mode>,
    host_side: Option<Side>,
    game: Option<Game>,
    step_info_showed: bool,
    throw_info_showed: bool
}

impl Controller {
    pub fn new() -> Self {        
        Self { 
            menu: Menu::new("SELECT GAME MODE:", MODES),
            scene: Scenes::GameMenu,
            mode: None,
            host_side: None,
            game: None,
            step_info_showed: false,
            throw_info_showed: false
        }
    }
}

// MARK: - Render
impl Controller {
    pub fn render(&mut self, frame: &mut Frame) {
        if frame.area().height < 15 || frame.area().width < 70 {  
          let resize_str = String::from("Make me larger, pls!");  
          Popup::render(resize_str, BorderStyle::Error, frame);
          return
        }
        
        match self.scene {
            Scenes::GameMenu => {
                Border::render(frame, BorderStyle::Menu);
                self.menu.render(frame)
            },
            Scenes::GameDeck => {                
                if let Some(g) = &mut self.game {
                    if self.step_info_showed {
                        // Step info
                        let is_white = g.step_of == Side::White;
                        let side_str = if is_white { SIDES[0] } else { SIDES[1] };
                        let step_srt = format!("Current step: {side_str}");
                        Popup::render(step_srt, BorderStyle::Step, frame);
                    } else if self.throw_info_showed {
                        // Throw info
                        let dice_1 = g.last_throw.dices[0].result;
                        let dice_2 = g.last_throw.dices[1].result;
                        let jackpot_res = if g.last_throw.is_jackpot { "Jackpot!" } else { "" };
                        let throw_str = format!("Throwed!\n Result: {dice_1} {dice_2} \n{jackpot_res}");
                        Popup::render(throw_str, BorderStyle::Throw, frame);
                    } else {
                        // Game field 
                        Border::render(frame, BorderStyle::Game);
                        Field::new().render(frame, &g.step_of, &g.deck);
                    }
                }
            }
        }
    }
}

// MARK: - Keys handling
impl Controller {
    pub fn handle_key(&mut self, key_event: KeyEvent) {
        match self.scene {
            Scenes::GameMenu => self.handle_menu_keys(key_event),
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
                if self.mode.is_none() {
                    self.on_select_mode();
                } else {
                    self.on_select_side();
                } 
            }
            _ => {}
        }
    }
    
    fn handle_game_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => {
                // ...
            },
            KeyCode::Down => {
                // ..
            },
            KeyCode::Char(' ') => {
                if self.step_info_showed {
                    self.step_info_showed = false;
                    self.throw_info_showed = true;
                }
            },
            KeyCode::Enter => {
                 if self.throw_info_showed {
                     self.throw_info_showed = false;
                 }
            }
            _ => {}
        }
    }
}

// MARK: - Actions
impl Controller {
    fn on_select_mode(&mut self) {
        self.mode = match self.menu.selected {
            "Multiplayer" => Some(Mode::Multiplayer),
            _ => Some(Mode::Singleplayer)
        };
        
        self.menu = Menu::new("SELECT YOUR SIDE:", SIDES);
    }
    
    fn on_select_side(&mut self) {
        self.host_side = match self.menu.selected {
            "Black" => Some(Side::Black),
            _ => Some(Side::White)
        };
        
        self.game = Some(Game::new(self.host_side.unwrap()));
        self.scene = Scenes::GameDeck;
        self.step_info_showed = true
    }
}