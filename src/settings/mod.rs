pub mod prelude {
    pub use super::camera_settings::CameraSettings;
    pub use super::objects_settings::{ObjectSettings, ObjectsSettings};
    pub use super::player_settings::PlayerSettings;
    pub use super::Settings;
}

pub mod camera_settings;
pub mod entity_components;
pub mod objects_settings;
pub mod player_settings;

use crate::resource;
use deathframe::amethyst;
use prelude::*;
use std::fmt;
use std::fs::File;

pub struct Settings {
    pub camera: CameraSettings,
    pub player: PlayerSettings,
    pub objects: ObjectsSettings,
}

impl Settings {
    pub fn load() -> deathframe::amethyst::Result<Self> {
        Ok(Self {
            camera: load_settings("settings/camera.ron")?,
            player: load_settings("settings/player.ron")?,
            objects: load_settings("settings/objects.ron")?,
        })
    }
}

fn load_settings<S, P>(path: P) -> amethyst::Result<S>
where
    for<'de> S: serde::Deserialize<'de>,
    P: fmt::Display,
{
    let file = File::open(resource(path.to_string()))?;
    Ok(ron::de::from_reader(file).map_err(|e| {
        amethyst::Error::from_string(format!(
            "Failed parsing RON settings file: {}\n{:#?}",
            path, e
        ))
    })?)
}
