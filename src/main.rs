mod core;
mod process;
// mod tools;

use crate::process::controller::Controller;

fn main() {
    let controller = Controller::new();
    controller.start();
}
