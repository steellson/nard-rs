use ratatui::{
    Frame,
    text::Text,
    layout::Constraint,
    style::{Modifier, Style, Stylize, palette::tailwind},
    widgets::{Cell, HighlightSpacing, Row, Table, TableState},
};

const MAX_LENGTH: usize = 2;

pub enum NavDirection {
    Up,
    Down,
}

pub struct Menu {
    pub selected: &'static str,
    header: &'static str,
    state: TableState,
    items: [&'static str; MAX_LENGTH],
}

impl Menu {
    pub fn new(
        header: &'static str, 
        items: [&'static str; MAX_LENGTH]
    ) -> Self {
        Self {
            header: header,
            selected: items[0],
            state: TableState::default().with_selected(0),
            items: items,
        }
    }

    pub fn select_row(&mut self, direction: NavDirection) {
        let item = match self.state.selected() {
            Some(i) => match direction {
                NavDirection::Up => {
                    if i >= self.items.len() - 1 { 0 } else { i + 1 }
                }
                NavDirection::Down => {
                    if i == 0 { self.items.len() - 1 } else { i - 1 }
                }
            },
            None => 0,
        };
        self.selected = self.items[item];
        self.state.select(Some(item));
    }
}

// MARK: - Render
impl<'a> Menu {
    pub fn render(&mut self, frame: &'a mut Frame) {
        let header_style = Style::default()
            .fg(tailwind::BLACK)
            .bg(tailwind::WHITE);
        
        let selected_cell_style = Style::default()
            .fg(tailwind::WHITE);

        let header = [self.header]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let rows: Vec<Row> = self
            .items
            .iter()
            .map(|content| {
                let text = Text::from(format!("\n{content}\n"))
                    .green()
                    .centered();
                let style = Style::new()
                    .fg(tailwind::WHITE)
                    .bg(tailwind::BLACK);
                Row::new(vec![Cell::from(text)])
                    .style(style)
                    .height(4)
            })
            .collect();

        let bar = " █ ";
        let t = Table::new(rows, [Constraint::Min(30)])
            .header(header)
            .cell_highlight_style(selected_cell_style)
            .highlight_symbol(Text::from(vec![
                "".into(),
                bar.green().into(),
                bar.green().into(),
                "".into(),
            ]))
            .bg(tailwind::BLACK)
            .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(t, frame.area(), &mut self.state);
    }
}
