use crate::components::prelude::Size;

#[derive(Deserialize)]
pub struct CameraSettings {
    pub z:    f32,
    pub size: Size,
}
