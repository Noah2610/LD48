pub mod data;
pub mod objects;
pub mod tiles;

use crate::components::prelude::Size;
use crate::resources::prelude::ZoneHeight;
use crate::settings::zones_settings::SegmentId;
use amethyst::ecs::{World, WorldExt};
use data::*;
use deathframe::amethyst;
use std::fs::File;
use std::path::PathBuf;

#[derive(Deserialize, Clone)]
pub enum TileType {
    #[serde(rename = "")]
    Empty,
}

#[derive(Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub enum ObjectType {
    Player,
    Solid,
}

pub fn load_level(filepath: PathBuf) -> amethyst::Result<DataLevel> {
    let level_file = File::open(filepath)?;
    let level_data = serde_json::de::from_reader::<_, DataLevel>(level_file)?;
    Ok(level_data)
}

pub fn build_level(
    world: &mut World,
    level_data: DataLevel,
    segment_id: SegmentId,
) -> amethyst::Result<()> {
    let offset_y = world.read_resource::<ZoneHeight>().height;

    let level_size =
        Size::new(level_data.level.size.w, level_data.level.size.h);
    let tile_size =
        Size::new(level_data.level.tile_size.w, level_data.level.tile_size.h);

    let segment_entity = objects::build_segment_collision(
        world,
        level_size.clone(),
        segment_id.clone(),
        offset_y,
    );

    tiles::build_tiles(
        world,
        level_data.tiles,
        tile_size,
        segment_id.clone(),
        segment_entity,
    )?;
    objects::build_objects(
        world,
        level_data.objects,
        level_size.clone(),
        segment_id,
        segment_entity,
    )?;

    world.write_resource::<ZoneHeight>().height += level_size.h;

    world.maintain();

    Ok(())
}
