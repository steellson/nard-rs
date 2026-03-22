use crate::core::{mode::Mode, side::Side};

#[derive(PartialEq)]
pub enum Flow {
    Mode,
    Side,
    Step,
    Throw,
    Select,
    Apply
}

pub struct State {
    pub flow: Flow,
    pub mode: Mode,
    pub host_side: Side
}

impl State {
    pub fn new(
        flow: Flow, 
        mode: Mode, 
        host_side: Side
    ) -> Self {
        Self {
            flow: flow,
            mode: mode,
            host_side: host_side
        }
    }

    pub fn switch_mode(&mut self) {
        self.mode = self.mode.inverted();
    }

    pub fn switch_host_side(&mut self) {
        self.host_side = self.host_side.inverted();
    }

    pub fn next(&mut self) {
        self.flow = match self.flow {
            Flow::Mode => Flow::Side,
            Flow::Side => Flow::Step,
            Flow::Step => Flow::Throw,
            Flow::Throw => Flow::Select,
            Flow::Select => Flow::Apply,
            Flow::Apply => Flow::Step
        };
    }
}