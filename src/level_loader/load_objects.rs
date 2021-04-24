use super::data::*;
use super::ObjectType;
use crate::components::prelude::*;
use crate::resource;
use crate::settings::entity_components::add_components_to_entity;
use crate::settings::prelude::*;
use amethyst::ecs::{Builder, Entity, World, WorldExt};
use deathframe::amethyst;
use deathframe::core::geo::prelude::Rect;
use deathframe::resources::SpriteSheetHandles;
use std::path::PathBuf;

pub fn load_objects(
    world: &mut World,
    objects: Vec<DataObject>,
    level_size: Size,
) -> amethyst::Result<()> {
    let objects_settings = (*world.read_resource::<ObjectsSettings>()).clone();

    for object in objects {
        let transform = {
            let mut transform = Transform::default();
            transform.set_translation_x(object.pos.x);
            transform.set_translation_y(object.pos.y);
            if let Some(z) = object.props.get("z").and_then(|val| val.as_f64())
            {
                transform.set_translation_z(z as f32);
            }
            transform
        };
        let size = Size::new(object.size.w, object.size.h);

        match &object.object_type {
            ObjectType::Player => {
                let player_entity = build_player(world, transform, size);
                let _ = build_camera(world, player_entity, level_size.clone());
            }

            object_type => {
                if let Some(object_settings) =
                    objects_settings.objects.get(object_type)
                {
                    let mut entity_builder = world
                        .create_entity()
                        .with(transform)
                        .with(size.clone())
                        .with(Object::from(object_type.clone()));

                    entity_builder = add_components_to_entity(
                        entity_builder,
                        object_settings.components.clone(),
                        Some(size),
                    );

                    entity_builder.build();
                } else {
                    eprintln!(
                        "[WARNING]\n    No settings for object: {:?}",
                        object_type
                    );
                }
            }
        }
    }
    Ok(())
}

fn build_player(world: &mut World, transform: Transform, size: Size) -> Entity {
    let sprite_render = {
        let sprite_sheet = world
            .write_resource::<SpriteSheetHandles<PathBuf>>()
            .get_or_load(resource("spritesheets/player.png"), world);
        SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        }
    };

    let settings = (*world.read_resource::<PlayerSettings>()).clone();

    let mut entity_builder = world
        .create_entity()
        .with(transform)
        .with(size.clone())
        .with(Transparent)
        .with(ScaleOnce::default())
        .with(Object::from(ObjectType::Player))
        .with(Player::default())
        .with(Velocity::default())
        .with(sprite_render);

    entity_builder = add_components_to_entity(
        entity_builder,
        settings.components.clone(),
        Some(size),
    );

    entity_builder.build()
}

pub fn build_camera(
    world: &mut World,
    player: Entity,
    level_size: Size,
) -> amethyst::Result<()> {
    use amethyst::renderer::Camera;
    use amethyst::utils::ortho_camera::{
        CameraNormalizeMode, CameraOrtho, CameraOrthoWorldCoordinates,
    };

    let settings = (*world.read_resource::<CameraSettings>()).clone();

    let size = settings.size;

    let camera = Camera::standard_2d(size.w, size.h);
    let mut camera_ortho =
        CameraOrtho::normalized(CameraNormalizeMode::Contain);
    let half_size = size.half();
    camera_ortho.world_coordinates = CameraOrthoWorldCoordinates {
        top: half_size.h,
        bottom: -half_size.h,
        left: -half_size.w,
        right: half_size.w,
        near: 0.0,
        far: 20.0,
    };

    let level_center = level_size.half();
    let mut transform = Transform::default();
    transform.set_translation_xyz(level_center.w, level_center.h, settings.z);

    world
        .create_entity()
        .with(Follow::new(player))
        .with(Confined::from(Rect {
            top: level_size.h,
            bottom: 0.0,
            left: 0.0,
            right: level_size.w,
        }))
        .with(transform)
        .with(size)
        .with(camera)
        .with(camera_ortho)
        .build();

    Ok(())
}
