use super::{ObjectType, TileType};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct DataLevel {
    pub level: DataLevelInfo,
    pub tiles: Vec<DataTile>,
    pub objects: Vec<DataObject>,
}

#[derive(Deserialize)]
pub struct DataLevelInfo {
    pub size: DataSize,
    pub tile_size: DataSize,
}

#[derive(Deserialize)]
pub struct DataObject {
    #[serde(rename = "type")]
    pub object_type: ObjectType,
    pub pos: DataPos,
    pub size: DataSize,
    pub props: DataProps,
}

#[derive(Deserialize)]
pub struct DataTile {
    pub id: usize,
    #[serde(rename = "type")]
    pub tile_type: TileType,
    pub ts: String,
    pub pos: DataPos,
    pub props: DataProps,
}

#[derive(Deserialize)]
pub struct DataPos {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize)]
pub struct DataSize {
    pub w: f32,
    pub h: f32,
}

pub type DataProps = HashMap<String, serde_json::Value>;
