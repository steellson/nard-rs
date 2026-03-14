use ratatui::{
    Frame,
    text::Line,
    widgets::Block,
    style::{palette::tailwind, Stylize},
    symbols::border,
};

pub enum BorderStyle {
    Menu,
    Game, 
    Step,
    Throw,
    Error
}

pub struct Border {}

// MARK: - Render
impl<'a> Border {
    pub fn render(frame: &'a mut Frame, style: BorderStyle) {
        let title = Line::from(" NARD-rS ".green().bold());

        let instructions = match style {
            BorderStyle::Menu => Line::from(vec![
                " Navigation ".into(),
                "⬆️ or ⬇️".bold(),
                " ---------- ".bold(),
                " Quit ".into(),
                "[Q] ".green().bold(),
            ]),
            BorderStyle::Game => Line::from(vec![
                " Navigation ".into(),
                "⬆️ or ⬇️".bold(),
                " -------- ".bold(),
                " Apply step ".into(),
                "[Enter]".green().bold(),
                " -------- ".bold(),
                " Quit ".into(),
                "[Q] ".green().bold(),
            ]),
            BorderStyle::Step => Line::from(vec![
                " Throw! ".into(),
                "[Space]".green().bold(),
                " ---------- ".bold(),
                " Quit ".into(),
                "[Q] ".green().bold(),
            ]),
            BorderStyle::Throw => Line::from(vec![
                " Continue ".into(),
                "[Enter]".green().bold(),
                " ---------- ".bold(),
                " Quit ".into(),
                "[Q] ".green().bold(),
            ]),
            BorderStyle::Error => Line::from(vec![
                " Quit ".into(),
                "[Q] ".green().bold(),
            ])
        };

        let border = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .bg(tailwind::BLACK)
            .border_set(border::ROUNDED);

        frame.render_widget(border, frame.area());
    }
}
