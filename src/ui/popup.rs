use ratatui::Frame;
use ratatui::widgets::Block;
use ratatui::style::{Style, Stylize, palette::tailwind};

use crate::ui::border::{Border, BorderStyle};


pub struct Popup {}

impl Popup {
    pub fn render(msg: &str, frame: &mut Frame) {
        let area = frame.area();
        
        // Game border
        Border::render(frame, BorderStyle::Error);
        
        // Popup border
        let popup_rect = ratatui::layout::Rect::new(
            area.width / 4, 
            1,
            area.width / 2,
            area.height - 2
        );
        let popup_border_style = Style::default()
            .bg(tailwind::BLACK);
        let popup_border = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(popup_border_style);
        
        frame.render_widget(popup_border, popup_rect);
        
        // Text 
        let text_rect = ratatui::layout::Rect::new(
            popup_rect.width / 2 + 1, 
            popup_rect.height / 2 + 1,
            popup_rect.width - 2,
            popup_rect.height - 2
        );
        let error_text = ratatui::widgets::Paragraph::new(msg.green())
            .centered()
            .wrap(ratatui::widgets::Wrap { trim: true });
        
        frame.render_widget(error_text, text_rect);
    }
}