use super::component_prelude::*;
use crate::level_loader::ObjectType;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Object {
    pub object_type: ObjectType,
}

impl From<ObjectType> for Object {
    fn from(object_type: ObjectType) -> Self {
        Self { object_type }
    }
}
