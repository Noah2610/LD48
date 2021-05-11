// resources/settings/savefile.ron

use deathframe::amethyst;
use std::fs::create_dir_all;
use std::path::PathBuf;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SavefileSettings {
    pub savefile_name: String,
}

impl SavefileSettings {
    pub fn savefile_path(&self) -> amethyst::Result<PathBuf> {
        const APP_NAME: &str = env!("CARGO_PKG_NAME");

        if let Some(mut path) = dirs::data_local_dir() {
            path.push(APP_NAME);
            if !path.exists() {
                if let Err(e) = create_dir_all(&path) {
                    return Err(amethyst::Error::from_string(format!(
                        "Couldn't create data directory for \
                         savefile:\n{:?}\n{:?}",
                        &path, e
                    )));
                }
            }
            path.push(&self.savefile_name);
            Ok(path)
        } else {
            Err(amethyst::Error::from_string(
                "Couldn't find data directory for savefile",
            ))
        }
    }
}
