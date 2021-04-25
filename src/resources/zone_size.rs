#[derive(Default)]
pub struct ZoneSize {
    pub width:  f32,
    pub height: f32,
}

impl ZoneSize {
    pub fn reset(&mut self) {
        // self.width = 0.0;
        self.height = 0.0;
    }
}
