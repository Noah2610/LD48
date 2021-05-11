use crate::settings::prelude::ZoneId;
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize)]
pub struct Highscore {
    pub highscore: usize,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Highscores {
    pub progression: Option<Highscore>,
    pub infinite:    HashMap<ZoneId, Highscore>,
}
