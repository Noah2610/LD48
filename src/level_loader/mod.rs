pub mod data;
pub mod objects;
pub mod tiles;

use crate::components::prelude::Size;
use amethyst::ecs::World;
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
) -> amethyst::Result<()> {
    let level_size =
        Size::new(level_data.level.size.w, level_data.level.size.h);
    let tile_size =
        Size::new(level_data.level.tile_size.w, level_data.level.tile_size.h);

    tiles::build_tiles(world, level_data.tiles, tile_size)?;
    objects::build_objects(world, level_data.objects, level_size.clone())?;

    Ok(())
}
