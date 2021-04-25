// resources/settings/camera.ron

use crate::components::prelude::Size;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct CameraSettings {
    pub z:             f32,
    pub size:          Size,
    pub follow_offset: (f32, f32),
}
