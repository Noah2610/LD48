use crate::level_loader::data::DataLevel;
use crate::level_loader::load_level;
use crate::resource;
use crate::settings::zones_settings::{SegmentId, ZoneId, ZonesSettings};
use rand::prelude::SliceRandom;
use std::collections::HashMap;

const KEEP_COUNT_SEGMENTS_LOADED: usize = 2;

#[derive(Default)]
pub struct ZonesManager {
    current_zone:        Option<ZoneState>,
    last_staged_segment: Option<SegmentId>,
    staged_segments:     Vec<SegmentId>,
    levels:              HashMap<SegmentId, DataLevel>,
}

#[derive(Debug)]
struct ZoneState {
    pub id:                    ZoneId,
    pub total_segments_loaded: usize,
}

impl From<ZoneId> for ZoneState {
    fn from(id: ZoneId) -> Self {
        Self {
            id,
            total_segments_loaded: 0,
        }
    }
}

impl ZonesManager {
    pub fn set_zone(&mut self, zone_id: ZoneId) {
        self.current_zone = Some(zone_id.into());
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
            self.current_zone
                .as_mut()
                .expect(
                    "Should have current zone, if next segment could be found",
                )
                .total_segments_loaded += 1;
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
            .and_then(|current_zone| {
                settings
                    .zones
                    .get(&current_zone.id)
                    .map(|settings| (current_zone, settings))
            })
            .map(|(current_zone, zone_settings)| {
                let should_load_final_segment = zone_settings
                    .total_segments
                    .map(|total_segments| {
                        current_zone.total_segments_loaded >= total_segments
                    })
                    .unwrap_or(false);
                if should_load_final_segment {
                    &zone_settings.final_segment
                } else {
                    self.last_staged_segment
                        .as_ref()
                        .and_then(|current_segment| {
                            zone_settings.segments.get(current_segment)
                        })
                        .unwrap_or(&zone_settings.first_segment)
                }
            })
            .and_then(|possible_segments| {
                possible_segments.choose(&mut rng).map(ToString::to_string)
            })
    }
}
