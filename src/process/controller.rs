use ratatui::Frame;
use crossterm::event::{KeyCode, KeyEvent};
use super::state::State;

use crate::{core::throw::Throw, ui::{
    menu::Menu, 
    popup::Popup,
    field::Field,
    border::{Border, BorderStyle}
}};
use crate::core::{
    game::Game,
    modes::Modes,
    sides::Sides
};

pub struct Controller {
    modes: Modes,
    sides: Sides,
    state: State,
    game: Option<Game>
}

impl Controller {
    pub fn new() -> Self {        
        Self { 
            modes: Modes::new(),
            sides: Sides::new(),
            state: State::new(),
            game: None
        }
    }
}

// MARK: - Render
impl Controller {
    pub fn render(&self, frame: &mut Frame) {        
        // Prevent small screen size
        if frame.area().height < 15 || frame.area().width < 70 {  
          let resize_str = String::from("Make me larger, pls!");  
          Popup::render(resize_str, BorderStyle::Error, frame);
          return
        }
        // Select mode
        if !self.state.mode_selected {
            self.render_mode_menu(frame);
            return
        }
        // Select host side
        if !self.state.side_selected {
            self.render_side_menu(frame);
            return
        }
        // Game process
        if let Some(g) = &self.game {
            // Step info
            if self.state.step_info_showed {
                self.render_step_info(frame);
                return
            } 
            // Throw info
            if self.state.throw_info_showed {
                self.render_throw_info(frame, &g.last_throw);
                return
            }
            // Game field 
            let field = Field::new(g.deck);
            field.render(frame);
            Border::render(frame, BorderStyle::Game);
        }
    }
    
    fn render_mode_menu(&self, frame: &mut Frame) {
        let values = self.modes.source.map(|m| 
            String::from(m.raw_value())
        ).to_vec();
        let selected = self.modes.selected;
        let menu = Menu::new("SELECT GAME MODE:", selected, values);
        menu.render(frame);
        Border::render(frame, BorderStyle::Menu);
    }
    
    fn render_side_menu(&self, frame: &mut Frame) {
        let values = self.sides.source.map(|m|
            format!("{} chips", String::from(m.raw_value()))
        ).to_vec();
        let selected = self.sides.selected;
        let menu = Menu::new("SELECT YOUR SIDE:", selected, values);
        menu.render(frame);
        Border::render(frame, BorderStyle::Menu);
    }
    
    fn render_step_info(&self, frame: &mut Frame) {
        let side_str = self.sides.source[self.sides.selected].raw_value();
        let step_srt = format!("Current step: {side_str}");
        Popup::render(step_srt, BorderStyle::Step, frame);
    }
    
    fn render_throw_info(&self, frame: &mut Frame, throw: &Throw) {
        let dice_1 = throw.dices[0].result;
        let dice_2 = throw.dices[1].result;
        let jackpot_res = if throw.is_jackpot { "Jackpot!" } else { "" };
        let throw_str = format!("Throwed!\n Result: {dice_1} {dice_2} \n{jackpot_res}");
        Popup::render(throw_str, BorderStyle::Throw, frame);
    }
}

// MARK: - Keys handling
impl Controller {
    pub fn handle_key(&mut self, key_event: KeyEvent) {
        if !self.state.mode_selected {
            self.handle_mode_menu_keys(key_event);
            return
        }
        if self.state.mode_selected && !self.state.side_selected {
            self.handle_side_menu_keys(key_event);
            return
        }
        if self.state.mode_selected && self.state.side_selected {
            self.handle_game_keys(key_event);
            return
        }
    }
    
    fn handle_mode_menu_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => { self.modes.toggle_selected(); },
            KeyCode::Down => { self.modes.toggle_selected() },
            KeyCode::Enter => {
                self.state.mode_selected = true;
            }
            _ => {}
        }
    }
    
    fn handle_side_menu_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => { self.sides.toggle_selected(); },
            KeyCode::Down => { self.sides.toggle_selected() },
            KeyCode::Enter => {
                let selected_side = self.sides.source[self.sides.selected];
                self.game = Some(Game::new(selected_side));
                self.state.side_selected = true;
                self.state.step_info_showed = true;
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
                if self.state.step_info_showed {
                    self.game.as_mut().unwrap().throw();
                    self.state.step_info_showed = false;
                    self.state.throw_info_showed = true;
                }
            },
            KeyCode::Enter => {
                 if self.state.throw_info_showed {
                     self.state.throw_info_showed = false;
                 }
            }
            _ => {}
        }
    }
}