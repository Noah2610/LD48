use crate::level_loader::data::DataLevel;
use crate::settings::zones_settings::{SegmentId, ZoneId};
use amethyst::ecs::{World, WorldExt};
use deathframe::amethyst;
use std::collections::HashMap;

const KEEP_COUNT_SEGMENTS_LOADED: usize = 2;

#[derive(Default)]
pub struct ZonesManager {
    pub loaded_zones: Vec<Zone>,
    pub levels:       HashMap<SegmentId, DataLevel>,
}

pub struct Zone {
    pub id: ZoneId,
}

impl ZonesManager {
    // pub fn update(&self, world: &mut World) {
    //     if self.should_load_next_segment(&*world) {}
    // }

    // fn should_load_next_segment(&self, world: &World) -> bool {
    //     if self.loaded_zones.len() < KEEP_COUNT_SEGMENTS_LOADED {
    //         return true;
    //     }

    //     unimplemented!()
    // }
}
