use ratatui::{
    Frame, layout::{Constraint, Flex, HorizontalAlignment, Layout, Offset, Rect}, symbols::line, text::Text
};

use crate::core::{
    side::Side,
    deck::{Deck, SIDE_CHIPS}, row::Row, sector::Placement
};

pub struct Field {
     border: &'static str,
     empty_char: &'static str,
     sector_layout: Layout
}

impl Field {
    pub fn new() -> Self {
        Field {
            border: "I",
            empty_char: "-",
            sector_layout: Layout::vertical([
                Constraint::Length(1); 6
            ]).flex(Flex::SpaceBetween)
        }
    }
}

// MARK: - Render
impl<'a> Field {
    pub fn render(
        &mut self, frame: &'a mut Frame,
        deck: &Deck
    ) {
        let area = frame.area();
        let width = area.width;
        let height = area.height;
        let paddings = 2;

        // Deck separator
        let mut separator = Vec::new();
        for _ in 1..=width - paddings {
            separator.push("=");
        }

        frame.render_widget(
            Text::from(separator.join("")),
            area.offset(Offset { x: 1, y: 0 })
                .centered_vertically(Constraint::Length(1)),
        );

        // Sectors and lines
        let content_height = height - paddings;
        let field = Rect::new(1, 1, width - paddings, content_height);
        let side_layout = Layout::vertical([
            Constraint::Length(content_height / 2); 2
        ]).spacing(1); 
        
        for sector in &deck.sectors {
            match sector.placement {
                Placement::A => {
                    for (index, row) in sector.rows.iter().enumerate() {
                        frame.render_widget(
                            &self.draw_line(&row, false),
                            self.sector_layout.split(side_layout.split(field)[0])[index],
                        );
                    }
                },
                Placement::B => {
                    for (index, row) in sector.rows.iter().enumerate() {
                        frame.render_widget(
                            &self.draw_line(&row, false),
                            self.sector_layout.split(side_layout.split(field)[1])[index],
                        );
                    }
                },
                Placement::C => {
                    let mut rows = sector.rows;
                    rows.reverse();
                    
                    for (index, row) in rows.iter().enumerate() {
                        frame.render_widget(
                            &self.draw_line(&row, true),
                            self.sector_layout.split(side_layout.split(field)[1])[index],
                        );
                    }
                },
                Placement::D => {
                    let mut rows = sector.rows;
                    rows.reverse();
                    
                    for (index, row) in rows.iter().enumerate() {
                        frame.render_widget(
                            &self.draw_line(&row, true),
                            self.sector_layout.split(side_layout.split(field)[0])[index],
                        );
                    }
                }
            }
        }
    }

    fn draw_line(&self, row: &Row, right_aligned: bool) -> Text<'a> {
        let mut line = self.line(row);
        
        match row.chips {
            Some(chips) => {   
                let chip = if chips[0].side == Side::White { 
                    "⚪️"
                } else { 
                    "⚫️"
                };
             
                for index in 0..=SIDE_CHIPS - 1 {
                    if index <= SIDE_CHIPS - 1 {
                        line[index] = chip
                    }
                }
            },
            None => {}
        }
        
        if right_aligned {
            line.reverse();
            Text::from(format!("{}{}", line.join(""), self.border))
                .alignment(HorizontalAlignment::Right)
        } else {
            Text::from(format!("{}{}", self.border, line.join("")))
        }
    }
    
    fn line(&self, row: &Row) -> [&str; SIDE_CHIPS] {
        let mut content = [self.empty_char; SIDE_CHIPS];
        
        match row.chips {
            Some(chips) => {                
                for index in 0..=SIDE_CHIPS - 1 {
                    if index <= SIDE_CHIPS - 1 {
                        content[index] = if chips[0].side == Side::White {
                            "⚪️"
                        } else {
                            "⚫️"
                        };
                    }
                }
                content
                
            },
            None => { content }
        }
    }
}
