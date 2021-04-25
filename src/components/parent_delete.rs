use super::component_prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct ParentDelete(pub Entity);
