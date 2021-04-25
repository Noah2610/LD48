pub mod data;
pub mod objects;
pub mod tiles;

use crate::components::prelude::Size;
use crate::resources::prelude::{ZoneSize, ZonesManager};
use crate::settings::zones_settings::{SegmentId, ZonesSettings};
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
    Portal,
    Obstacle,
    Custom(String),
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
    let is_final_segment = {
        let zones_manager = world.read_resource::<ZonesManager>();
        let zones_settings = world.read_resource::<ZonesSettings>();
        if let Some(current_zone) = zones_manager.current_zone() {
            zones_settings
                .zones
                .get(current_zone)
                .map(|zone_settings| {
                    zone_settings.final_segment.contains(&segment_id)
                })
                .unwrap_or(false)
        } else {
            false
        }
    };

    let level_size =
        Size::new(level_data.level.size.w, level_data.level.size.h);
    let tile_size =
        Size::new(level_data.level.tile_size.w, level_data.level.tile_size.h);

    // let offset_y = world.read_resource::<ZoneSize>().height;
    let offset_y = {
        let mut zone_size = world.write_resource::<ZoneSize>();
        zone_size.height += level_size.h;
        zone_size.height
    };

    let segment_entity = objects::build_segment_collision(
        world,
        level_size.clone(),
        segment_id.clone(),
        is_final_segment,
        offset_y,
    );

    tiles::build_tiles(
        world,
        level_data.tiles,
        tile_size,
        segment_id.clone(),
        segment_entity,
        offset_y,
    )?;
    objects::build_objects(
        world,
        level_data.objects,
        segment_id,
        segment_entity,
        offset_y,
    )?;

    // {
    //     let mut zone_size = world.write_resource::<ZoneSize>();
    //     zone_size.width = level_size.w;
    //     zone_size.height += level_size.h;
    // }

    world.maintain();

    Ok(())
}
