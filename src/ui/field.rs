use ratatui::Frame;

pub struct Field {}

impl Field {
    pub fn new() -> Self { Field {} }
}

// MARK: - Render
impl<'a> Field {
    pub fn render(&mut self, frame: &'a mut Frame) {
        // ...
    }
}
