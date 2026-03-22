use ratatui::Frame;
use crossterm::event::{KeyCode, KeyEvent};
use super::state::{State, Flow};

use crate:: ui::{
    menu::Menu,
    popup::Popup,
    field::Field,
    border::{Border, BorderStyle},
};
use crate::core::{
    game::Game,
    throw::Throw,
    player::Player,
    step::NavDirection,
    mode::{Mode, ALL_MODES},
    side::{Side, ALL_SIDES},
};

pub struct Controller {
    state: State,
    game: Option<Game>
}

impl Controller {
    pub fn new() -> Self {  
        Self { 
            state: State::new(
                Flow::Mode,
                Mode::Singleplayer,
                Side::White
            ),
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
        if self.state.flow == Flow::Mode {
            self.render_mode_menu(frame);
            return
        }
        // Select side
        if self.state.flow == Flow::Side {
            self.render_side_menu(frame);
            return
        }
        // Game process
        if let Some(g) = &self.game {
            // Step info
            if self.state.flow == Flow::Step {
                self.render_step_info(frame, g);
                return
            } 
            // Throw info
            if self.state.flow == Flow::Throw {
                self.render_throw_info(frame, &g.step.throw);
                return
            }
            // Game field 
            let field = Field::new(g.deck);
            field.render(frame);
            Border::render(frame, BorderStyle::Game);
        }
    }

    fn render_mode_menu(&self, frame: &mut Frame) {
        let values: Vec<String> = ALL_MODES.iter()
            .map(|m| m.raw_value().to_string())
            .collect();
        let selected = self.state.mode.raw_value();
        let menu = Menu::new("SELECT GAME MODE:", selected, values);
        menu.render(frame);
        Border::render(frame, BorderStyle::Menu);
    }
 
    fn render_side_menu(&self, frame: &mut Frame) {
        let values: Vec<String> = ALL_SIDES.iter()
            .map(|m| m.raw_value().to_string())
            .collect();
        let selected = self.state.host_side.raw_value();
        let menu = Menu::new("SELECT YOUR SIDE:", selected, values);
        menu.render(frame);
        Border::render(frame, BorderStyle::Menu);
    }

    fn render_step_info(&self, frame: &mut Frame, game: &Game) {
        let side_str = game.step.side.raw_value();
        let step_str = format!("Current step: {side_str}");
        Popup::render(step_str, BorderStyle::Step, frame);
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
        match self.state.flow {
            Flow::Mode => self.handle_mode_menu_keys(key_event),
            Flow::Side => self.handle_side_menu_keys(key_event),
            _ => self.handle_game_keys(key_event)
        }
    }

    fn handle_mode_menu_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => self.state.switch_mode(),
            KeyCode::Down => self.state.switch_mode(),
            KeyCode::Enter => self.state.next(),
            _ => {}
        }
    }

    fn handle_side_menu_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => self.state.switch_host_side(),
            KeyCode::Down => self.state.switch_host_side(),
            KeyCode::Enter => {
                let host_side = self.state.host_side;
                let host = Player::new(true, host_side);
                let opposite = Player::new(false, host_side.inverted());
                self.game = Some(Game::new([host, opposite]));
                self.state.next();
            }
            _ => {}
        }
    }
    
    fn handle_game_keys(&mut self, key_event: KeyEvent) {
        if let Some(g) = &mut self.game {
            match key_event.code {
                KeyCode::Up => {
                    if self.state.flow == Flow::Select {
                        g.select_row(NavDirection::Up);
                        return
                    }
                    if self.state.flow == Flow::Apply {
                        g.select_chip(NavDirection::Up);
                        return
                    }
                },
                KeyCode::Down => {
                    if self.state.flow == Flow::Select {
                        g.select_row(NavDirection::Down);
                        return
                    }
                    if self.state.flow == Flow::Apply {
                        g.select_chip(NavDirection::Down);
                        return
                    }
                },
                KeyCode::Char(' ') => {
                    if self.state.flow == Flow::Step {
                        g.make_throw();
                        self.state.next();
                    }
                },
                KeyCode::Enter => {
                    if self.state.flow == Flow::Throw {
                        self.state.next();
                        return
                    }
                    if self.state.flow == Flow::Select {
                        self.state.next();
                        return
                    }
                    if self.state.flow == Flow::Apply {
                        g.apply_step();
                        self.state.next();
                        return
                    }
                }
                _ => {}
            }
        }
    }
}