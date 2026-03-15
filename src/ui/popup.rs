use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize, palette::tailwind},
    widgets::{Block, BorderType, Paragraph, Wrap}
};

use crate::ui::border::{Border, BorderStyle};

pub struct Popup {}

impl Popup {
    pub fn render(
        msg: String,
        border: BorderStyle, 
        frame: &mut Frame
    ) {
        let area = frame.area();
        
        // Game border
        Border::render(frame, border);
        
        // Popup border
        let popup_rect = Rect::new(
            area.width / 4, 
            1,
            area.width / 2,
            area.height - 2
        );
        let popup_border_style = Style::default()
            .bg(tailwind::BLACK);
        let popup_border = Block::bordered()
            .border_type(BorderType::Rounded)
            .style(popup_border_style);
        
        frame.render_widget(popup_border, popup_rect);
        
        // Text 
        let text_rect = Rect::new(
            popup_rect.width / 2 + 2, 
            popup_rect.height / 2 + 1,
            popup_rect.width - 2,
            popup_rect.height - 2
        );
        let text = Paragraph::new(msg.green())
            .centered()
            .wrap(Wrap { trim: true });
        
        frame.render_widget(text, text_rect);
    }
}