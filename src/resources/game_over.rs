#[derive(Default)]
pub struct GameOver(pub bool);

impl GameOver {
    pub fn reset(&mut self) {
        self.0 = false;
    }
}
