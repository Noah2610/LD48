use super::data::*;
use super::ObjectType;
use crate::components::prelude::*;
use crate::resource;
use amethyst::ecs::{Builder, World, WorldExt};
use deathframe::amethyst;
use deathframe::resources::SpriteSheetHandles;
use std::path::PathBuf;

pub fn load_objects(
    world: &mut World,
    objects: Vec<DataObject>,
    level_size: Size,
) -> amethyst::Result<()> {
    for object in objects {
        match &object.object_type {
            ObjectType::Player => {
                let transform = {
                    let mut transform = Transform::default();
                    transform.set_translation_x(object.pos.x);
                    transform.set_translation_y(object.pos.y);
                    if let Some(z) =
                        object.props.get("z").and_then(|val| val.as_f64())
                    {
                        transform.set_translation_z(z as f32);
                    }
                    transform
                };
                let size = Size::new(object.size.w, object.size.h);

                let sprite_render = {
                    let sprite_sheet = world
                        .write_resource::<SpriteSheetHandles<PathBuf>>()
                        .get_or_load(
                            resource("spritesheets/player.png"),
                            world,
                        );
                    SpriteRender {
                        sprite_sheet,
                        sprite_number: 0,
                    }
                };

                let entity = world
                    .create_entity()
                    .with(transform)
                    .with(size.clone())
                    .with(Transparent)
                    .with(ScaleOnce::default())
                    .with(Object::from(object.object_type))
                    .with(Player::default())
                    .with(Velocity::default())
                    .build();
            }
        }
    }
    Ok(())
}
