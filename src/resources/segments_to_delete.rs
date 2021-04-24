use crate::settings::zones_settings::SegmentId;
use std::collections::hash_set::{Drain, HashSet};

#[derive(Default)]
pub struct SegmentsToDelete {
    pub to_delete: HashSet<SegmentId>,
}

impl SegmentsToDelete {
    pub fn stage(&mut self, segment: SegmentId) {
        self.to_delete.insert(segment);
    }
}
