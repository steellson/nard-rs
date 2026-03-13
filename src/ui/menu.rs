use ratatui::{
    Frame, layout::{Constraint, HorizontalAlignment, Rect}, style::{Style, Stylize, palette::tailwind}, text::Text, widgets::{Cell, Row, Table, TableState}
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
        let area = frame.area();
        
        // Header
        frame.render_widget(
            Text::from(self.header.green().bold().italic()),
            area.centered(
                Constraint::Max(18),
                Constraint::Ratio(1, 2)
            )
        );
        
        // Table
        frame.render_stateful_widget(
            self.table(), 
            area.centered(
                Constraint::Max(18),
                Constraint::Ratio(1, 2)
            ),
            &mut self.state
        );
    }
    
    fn rows(&mut self) -> Vec<Row<'a>> {
        self
            .items
            .iter()
            .map(|content| {
                let text = Text::from(format!("\n{content}"))
                    .green()
                    .centered()
                    .alignment(HorizontalAlignment::Left);
                
                let style = Style::new()
                    .bg(tailwind::BLACK);
                
                Row::new(vec![Cell::from(text)])
                    .style(style)
                    .height(2)
            })
            .collect()
    }
    
    fn table(&mut self) -> Table<'a> {
        let highlited_style = Style::default()
            .fg(tailwind::WHITE)
            .rapid_blink();
        
        let highlited_symbol = Text::from(vec![ 
            "".into(),
            " █ ".into(),
            " █ ".into(),
            "".into(),
        ]);
        
        Table::new(self.rows(), [Constraint::Percentage(100)])
            .header(Row::new(Text::from("")))
            .row_highlight_style(highlited_style)
            .highlight_symbol(highlited_symbol)
            .bg(tailwind::BLACK)
    }
}