use crate::level_loader::data::DataLevel;
use crate::level_loader::load_level;
use crate::resource;
use crate::settings::zones_settings::{SegmentId, ZoneId, ZonesSettings};
use rand::prelude::SliceRandom;
use std::collections::HashMap;

const KEEP_COUNT_SEGMENTS_LOADED: usize = 2;

#[derive(Default)]
pub struct ZonesManager {
    current_zone:        Option<ZoneId>,
    last_staged_segment: Option<SegmentId>,
    staged_segments:     Vec<SegmentId>,
    levels:              HashMap<SegmentId, DataLevel>,
}

impl ZonesManager {
    pub fn set_zone(&mut self, zone_id: ZoneId) {
        self.current_zone = Some(zone_id);
    }

    pub fn levels_to_load(&mut self) -> Vec<(SegmentId, DataLevel)> {
        self.staged_segments
            .split_off(0)
            .into_iter()
            .filter_map(|segment| {
                self.get_level_or_load(segment.clone())
                    .map(|level| (segment, level))
            })
            .collect()
    }

    fn get_level_or_load(&mut self, segment: SegmentId) -> Option<DataLevel> {
        self.levels.get(&segment).map(Clone::clone).or_else(
            || match load_level(resource(format!("levels/zones/{}", &segment)))
            {
                Ok(level) => {
                    self.levels.insert(segment, level.clone());
                    Some(level)
                }
                Err(e) => {
                    eprintln!(
                        "[WARNING]\n    Failed parsing level JSON file:\n{:#?}",
                        e
                    );
                    None
                }
            },
        )
    }

    pub fn stage_next_segment(&mut self, settings: &ZonesSettings) {
        if let Some(next_segment) = self.get_next_segment(settings) {
            self.last_staged_segment = Some(next_segment.clone());
            self.staged_segments.push(next_segment);
        } else {
            self.last_staged_segment = None;
            eprintln!(
                "[WARNING]\n    ZonesManager couldn't find possible next \
                 segment, for zone and segment: {:?} {:?}",
                &self.current_zone, &self.last_staged_segment
            );
        }
    }

    fn get_next_segment(&self, settings: &ZonesSettings) -> Option<SegmentId> {
        let mut rng = rand::thread_rng();
        self.current_zone
            .as_ref()
            .and_then(|current_zone| dbg!(settings.zones.get(current_zone)))
            .map(|zone_settings| {
                dbg!(self
                    .last_staged_segment
                    .as_ref()
                    .and_then(|current_segment| {
                        zone_settings.segments.get(current_segment)
                    })
                    .unwrap_or(&zone_settings.first_segment))
            })
            .and_then(|possible_segments| {
                dbg!(possible_segments
                    .choose(&mut rng)
                    .map(ToString::to_string))
            })
    }

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
