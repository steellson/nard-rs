use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style,Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, StatefulWidget, Table, Widget, Row, TableState},
};

// MARK: - Border
#[derive(Debug)]
pub struct MenuBorder {}

impl MenuBorder {
    pub fn new() -> Self {
        Self {}
    }
}

// MARK: - Render border
impl Widget for &MenuBorder {
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

        let text = Text::from(
            vec![
                Line::from(vec![1337.to_string().yellow()])
            ]
        );

        Paragraph::new(text)
            .centered()
            .block(border)
            .render(area, buf);
    }
}

// MARK: - Selector
#[derive(Debug)]
pub struct MenuSelector {}

impl MenuSelector {
    pub fn new() -> Self { Self {} } 
}

// MARK: - Render selector
impl StatefulWidget for &MenuSelector {
    type State = TableState;
    
    fn render(
        self,
        area: Rect,
        buf: &mut Buffer, 
        state: &mut TableState
    ) {        
        let rows = [
            Row::new(vec!["Row11"]),
            Row::new(vec!["Row21"]),
            Row::new(vec!["Row31"]),
        ];
        let widths = [
            Constraint::Length(50),
            Constraint::Length(50),
            Constraint::Length(10),
        ];
        
        Table::new(rows, widths)
            .block(Block::new().title("Table"))
            .row_highlight_style(Style::new().reversed())
            .highlight_symbol(">>");
            // .render(area, buf, state);
    }
}