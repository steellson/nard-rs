use ratatui::{
    Frame,
    text::Text,
    style::Stylize,
    layout::{Constraint, Rect},
    widgets::{Cell, Row, Table}
};

pub struct Menu {
    header: &'static str,
    selected_index: usize,
    items: Vec<String>
}

impl Menu {
    pub fn new(
        header: &'static str, 
        selected_index: usize,
        items: Vec<String>
    ) -> Self {
        Self {
            header: header,
            selected_index: selected_index,
            items: items,
        }
    }
}

// MARK: - Render
impl<'a> Menu {
    pub fn render(self, frame: &'a mut Frame) {
        let width = frame.area().width;
        let height = frame.area().height;

        let header_lenght = self.header.len() as u16;
        let items_count = self.items.iter().count() as u16;
        let longest_item_count = self.items.iter()
            .map(|i| i.len())
            .max()
            .unwrap() as u16;

        // Header
        frame.render_widget(
            self.header(),
            Rect::new(
                width / 2 - (header_lenght / 2), 
                height / 2 - items_count - 1,
                header_lenght,
                2
            )
        );
        
        // List
        frame.render_widget(
            self.list(),
            Rect::new(
                width / 2 - (header_lenght / 2) + 1, 
                height / 2 - (items_count / 2) - 1,
                longest_item_count + 2,
                items_count * 2
            )
        );
    }
}

// MARK: - Elements
impl<'a> Menu {
    fn header(&self) -> Text<'a> {
        Text::from(self.header.green().bold().italic())
    }
    
    fn list(self) -> Table<'a> {
        Table::new(self.rows(), [Constraint::Percentage(100)])
            .header(Row::new(Text::from("".black())))
    }
    
    fn rows(self) -> Vec<Row<'a>> {
        self.items.iter().map(|content| {
            let text = if content == &self.items[self.selected_index] {
                Text::from(format!("█ {content}")).white()
            } else {
                Text::from(format!("  {content}")).green()
            };
            Row::new(vec![Cell::from(text)]).height(2)
        }).collect()
    }
}