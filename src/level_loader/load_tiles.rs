use super::data::*;
use crate::components::prelude::*;
use crate::resource;
use amethyst::ecs::{Builder, World, WorldExt};
use deathframe::amethyst;
use deathframe::resources::SpriteSheetHandles;
use std::path::PathBuf;

pub fn load_tiles(
    world: &mut World,
    tiles: Vec<DataTile>,
    tile_size: Size,
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

        let mut entity_builder = world
            .create_entity()
            .with(transform)
            .with(tile_size.clone())
            .with(sprite_render)
            .with(Transparent)
            .with(ScaleOnce::default())
            .with(Tile::default());

        entity_builder.build();
    }

    Ok(())
}
