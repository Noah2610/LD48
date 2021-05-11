use crate::settings::prelude::ZoneId;
use deathframe::amethyst;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize)]
pub struct Savefile {
    pub highscores: Highscores,
    pub unlocked:   HashSet<ZoneId>,

    #[serde(skip)]
    should_save: bool,
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

    pub fn save(&self, savefile_path: PathBuf) -> amethyst::Result<()> {
        let file = File::create(savefile_path)?;
        serde_json::ser::to_writer(file, self)?;
        Ok(())
    }

    pub fn should_save(&self) -> bool {
        self.should_save
    }

    pub fn unlock(&mut self, zone: ZoneId) -> bool {
        let did_update = self.unlocked.insert(zone);
        if did_update {
            self.should_save = true;
        }
        did_update
    }

    pub fn update_highscore_progression(&mut self, score: usize) -> bool {
        let highscore = self
            .highscores
            .progression
            .get_or_insert_with(Default::default);
        let did_update = score > highscore.highscore;
        if did_update {
            highscore.highscore = score;
            self.should_save = true;
        }
        did_update
    }

    pub fn update_highscore_infinite(
        &mut self,
        score: usize,
        zone: ZoneId,
    ) -> bool {
        let highscore = self.highscores.infinite.entry(zone).or_default();
        let did_update = score > highscore.highscore;
        if did_update {
            highscore.highscore = score;
            self.should_save = true;
        }
        did_update
    }
}
