use super::data::*;
use crate::components::prelude::*;
use crate::resource;
use crate::settings::zones_settings::SegmentId;
use amethyst::ecs::{Builder, Entity, World, WorldExt};
use deathframe::amethyst;
use deathframe::resources::SpriteSheetHandles;
use std::path::PathBuf;

pub fn build_tiles(
    world: &mut World,
    tiles: Vec<DataTile>,
    tile_size: Size,
    segment_id: SegmentId,
    segment_entity: Entity,
) -> amethyst::Result<()> {
    for tile in tiles {
        let transform = {
            let mut transform = Transform::default();
            transform.set_translation_x(tile.pos.x);
            transform.set_translation_y(tile.pos.y);
            if let Some(z) = tile.props.get("z").and_then(|val| val.as_f64()) {
                transform.set_translation_z(z as f32);
            }
            transform
        };

        let sprite_render = {
            let sprite_sheet = world
                .write_resource::<SpriteSheetHandles<PathBuf>>()
                .get_or_load(
                    resource(format!("spritesheets/tiles/{}", &tile.ts)),
                    world,
                );
            SpriteRender {
                sprite_sheet,
                sprite_number: tile.id,
            }
        };

        let entity_builder = world
            .create_entity()
            .with(transform)
            .with(tile_size.clone())
            .with(sprite_render)
            .with(Transparent)
            .with(ScaleOnce::default())
            .with(Tile::default())
            .with(BelongsToSegment(segment_id.clone()))
            .with(Parent::new(segment_entity));

        entity_builder.build();
    }

    Ok(())
}
