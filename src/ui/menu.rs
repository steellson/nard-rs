use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug)]
pub struct Menu {}

impl Menu {
    pub fn new() -> Self {
        Self {}
    }
}

// MARK: - Render
impl Widget for &Menu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" NARD-rS ".green().bold());

        let instructions = Line::from(vec![
            " Select ".into(),
            "⬆️ or ⬇️".bold(),
            " ---------- ".bold(),
            " Quit ".into(),
            "[Q] ".green().bold(),
        ]);

        let border = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::ROUNDED);

        let text = Text::from(vec![Line::from(vec![1337.to_string().yellow()])]);

        Paragraph::new(text)
            .centered()
            .block(border)
            .render(area, buf);
    }
}
