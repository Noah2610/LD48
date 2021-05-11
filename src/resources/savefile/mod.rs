use crate::settings::prelude::ZoneId;
use deathframe::amethyst;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize)]
pub struct Savefile {
    pub highscores: Highscores,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Highscores {
    pub progression: Option<Highscore>,
    pub infinite:    HashMap<ZoneId, Highscore>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Highscore {
    pub highscore: usize,
}

impl Savefile {
    pub fn load(savefile_path: PathBuf) -> amethyst::Result<Self> {
        if savefile_path.is_file() {
            let savefile_file = File::open(&savefile_path)
                .expect("Savefile file should exist at this point");
            Ok(serde_json::de::from_reader(savefile_file)?)
        } else {
            Err(amethyst::Error::from_string("Savefile doesn't exist"))
        }
    }
}
