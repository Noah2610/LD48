pub mod prelude {
    pub use super::camera_settings::CameraSettings;
    pub use super::Settings;
}

pub mod camera_settings;

use crate::resource;
use deathframe::amethyst;
use prelude::*;
use std::fmt;
use std::fs::File;

#[derive(Deserialize)]
pub struct Settings {
    pub camera: CameraSettings,
}

impl Settings {
    pub fn load() -> deathframe::amethyst::Result<Self> {
        Ok(Self {
            camera: load_settings("settings/camera.ron")?,
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
