use ratatui::{
    Frame,
    text::Text,
    layout::{Constraint, Layout, Offset, Rect, Flex},
};

use crate::core::{deck::Deck, sector::Placement};

pub struct Field {
     border: &'static str,
     line_spot: &'static str,
     sector_layout: Layout
}

impl Field {
    pub fn new() -> Self {
        Field {
            border: "I",
            line_spot: "-------------------------",
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
            for (index, _) in sector.rows.iter().enumerate() {
                match sector.placement {
                    Placement::A => {
                        frame.render_widget(
                            &self.aligned_line(false),
                            self.sector_layout.split(side_layout.split(field)[0])[index],
                        );
                    }
                    Placement::B => {
                        frame.render_widget(
                            &self.aligned_line(false),
                            self.sector_layout.split(side_layout.split(field)[1])[index],
                        );
                    }
                    Placement::C => {
                        frame.render_widget(
                            &self.aligned_line(true),
                            self.sector_layout.split(side_layout.split(field)[1])[index],
                        );
                    }
                    Placement::D => {
                        frame.render_widget(
                            &self.aligned_line(true),
                            self.sector_layout.split(side_layout.split(field)[0])[index],
                        );
                    }
                }
            }
        }
    }

    fn aligned_line(&self, right_aligned: bool) -> Text<'a> {
        if right_aligned {
            Text::from(format!("{}{}", self.line_spot, self.border))
                .alignment(ratatui::layout::HorizontalAlignment::Right)
        } else {
            Text::from(format!("{}{}", self.border, self.line_spot))
        }
    }
}
