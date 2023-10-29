pub struct Mode {
    pub toggle_mode: bool,
}

impl Mode {
    pub fn new(toggle_mode: bool) -> Mode {
        Mode { toggle_mode }
    }

    pub fn toggle(&mut self) {
        self.toggle_mode = !self.toggle_mode;
    }

    pub fn is_not_on(self) -> bool {
        !self.toggle_mode
    }
}
