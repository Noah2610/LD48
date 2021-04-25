use crate::level_loader::data::DataLevel;
use crate::level_loader::load_level;
use crate::resource;
use crate::settings::zones_settings::{SegmentId, ZoneId, ZonesSettings};
use rand::prelude::SliceRandom;
use replace_with::replace_with_or_abort;
use std::collections::HashMap;

const KEEP_COUNT_SEGMENTS_LOADED: usize = 2;

#[derive(Default)]
pub struct ZonesManager {
    current_zone:           Option<ZoneState>,
    last_staged_segment:    Option<SegmentId>,
    staged_segments:        Vec<SegmentId>,
    levels:                 HashMap<SegmentId, DataLevel>,
    segment_loading_locked: bool,
}

#[derive(Debug)]
struct ZoneState {
    pub id:                    ZoneId,
    pub order_idx:             usize,
    pub total_segments_loaded: usize,
}

impl ZoneState {
    pub fn new(id: ZoneId, order_idx: usize) -> Self {
        Self {
            id,
            order_idx,
            total_segments_loaded: 0,
        }
    }
}

impl ZonesManager {
    pub fn current_zone(&self) -> Option<&ZoneId> {
        self.current_zone.as_ref().map(|current| &current.id)
    }

    #[deprecated]
    pub fn set_zone(&mut self, zone_id: ZoneId) {
        self.current_zone = Some(ZoneState::new(zone_id, 0));
    }

    pub fn lock_segment_loading(&mut self) {
        self.segment_loading_locked = true;
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

    pub fn stage_next_zone(&mut self, settings: &ZonesSettings) {
        if let Some((next_zone, next_order_idx)) = self.get_next_zone(settings)
        {
            self.reset();
            self.current_zone = Some(ZoneState::new(next_zone, next_order_idx));
        } else {
            eprintln!("[WARNING]\n    There is no next zone to load!");
        }
    }

    fn get_next_zone(
        &self,
        settings: &ZonesSettings,
    ) -> Option<(ZoneId, usize)> {
        if let Some(current_zone) = self.current_zone.as_ref() {
            let order_idx = current_zone.order_idx + 1;
            settings
                .config
                .zone_order
                .get(order_idx)
                .map(|next| (next, order_idx))
        } else {
            settings.config.zone_order.first().map(|first| (first, 0))
        }
        .map(|(next, order_idx)| (next.clone(), order_idx))
    }

    fn reset(&mut self) {
        replace_with_or_abort(self, |_| ZonesManager {
            current_zone:           None,
            last_staged_segment:    None,
            staged_segments:        Vec::new(),
            levels:                 HashMap::new(),
            segment_loading_locked: false,
        });
    }

    pub fn stage_initial_segments(&mut self, settings: &ZonesSettings) {
        for _ in 0 .. KEEP_COUNT_SEGMENTS_LOADED {
            self.stage_next_segment(settings);
        }
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
        if self.segment_loading_locked {
            return None;
        }

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

    pub fn is_final_segment_loaded(&self, settings: &ZonesSettings) -> bool {
        self.current_zone
            .as_ref()
            .and_then(|current_zone| {
                settings
                    .zones
                    .get(&current_zone.id)
                    .map(|zone_settings| (current_zone, zone_settings))
            })
            .map(|(current_zone, zone_settings)| {
                zone_settings
                    .total_segments
                    .map(|total_segments| {
                        current_zone.total_segments_loaded > total_segments
                    })
                    .unwrap_or(false)
            })
            .unwrap_or(true)
    }
}
