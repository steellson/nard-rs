mod core;
mod process;
mod ui;

use crate::process::{app::App, controller::Controller};
use std::io;

fn main() -> io::Result<()> {
    let controller = Controller::new();
    let mut app = App::new(controller);
    ratatui::run(|terminal| app.run(terminal))
}
