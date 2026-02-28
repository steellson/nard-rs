use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use super::controller::Controller;

#[derive(Debug)]
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

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(
            &self.controller.menu, 
            frame.area()
        );
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => { self.exit = true },
            _ => {}
        }
    }
}