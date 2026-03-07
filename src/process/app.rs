use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use super::controller::Controller;

pub struct App {
    controller: Controller,
    exit: bool,
}

// MARK: - Lifecycle
impl App {
    pub fn new(controller: Controller) -> Self {
        Self { 
            controller: controller,
            exit: false
        }
    }
    
    pub fn run(
        &mut self, 
        terminal: &mut DefaultTerminal
    ) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.controller.render(frame);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                // Handling exit
                if key_event.code == KeyCode::Char('q') {
                    self.exit = true
                }
                // Handling other actions from controller
                self.controller.handle_key(key_event);
            },
            _ => {}
        };
        Ok(())
    }
}