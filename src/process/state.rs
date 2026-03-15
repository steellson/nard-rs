pub struct State {
    pub mode_selected: bool,
    pub side_selected: bool,
    pub step_info_showed: bool,
    pub throw_info_showed: bool
}

impl State {
    pub fn new() -> Self {
        Self {
            mode_selected: false,
            side_selected: false,
            step_info_showed: false,
            throw_info_showed: false
        }
    }
}