use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct Rotate {
    pub step: f32,
}
