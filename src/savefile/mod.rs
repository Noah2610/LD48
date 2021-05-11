pub mod highscore;

use deathframe::amethyst;
use highscore::Highscores;
use std::fs::File;
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize)]
pub struct Savefile {
    pub highscores: Highscores,
}

impl Savefile {
    pub fn load(savefile_path: PathBuf) -> amethyst::Result<Self> {
        if savefile_path.is_file() {
            let savefile_file = File::open(&savefile_path)
                .expect("Savefile file should exist at this point");
            Ok(serde_json::de::from_reader(savefile_file)?)
        } else {
            Err(amethyst::Error::from_string("Savefile path is not a file"))
        }
    }
}
