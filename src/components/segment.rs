use super::component_prelude::*;
use crate::settings::zones_settings::SegmentId;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Segment {
    pub id:               SegmentId,
    pub is_first_segment: bool,
    pub is_final_segment: bool,
}
