use crate::resources::prelude::{SongKey, SoundKey};
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AudioSettings {
    pub volume: f32,
    pub bgm:    HashMap<SongKey, AudioBgmSettings>,
    pub sfx:    HashMap<SoundKey, AudioSfxSettings>,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AudioBgmSettings {
    pub file: String,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AudioSfxSettings {
    pub file: String,
}
