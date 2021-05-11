// resources/settings/savefile.ron

use std::fs::create_dir_all;
use std::path::PathBuf;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SavefileSettings {
    pub savefile_name: String,
}

impl SavefileSettings {
    pub fn savefile_path(&self) -> Option<PathBuf> {
        const APP_NAME: &str = env!("CARGO_PKG_NAME");

        if let Some(mut path) = dirs::data_local_dir() {
            path.push(APP_NAME);
            if !path.exists() {
                if let Err(e) = create_dir_all(&path) {
                    eprintln!(
                        "[WARNING]\n    Couldn't create data directory for \
                         savefile:\n{:?}\n{:?}",
                        &path, e
                    );
                    return None;
                }
            }
            path.push(&self.savefile_name);
            Some(path)
        } else {
            eprintln!(
                "[WARNING]\n    Couldn't find data directory for savefile"
            );
            None
        }
    }
}
