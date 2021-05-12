use crate::settings::prelude::ZoneId;

#[derive(Default)]
pub struct SelectedZone(pub Option<(usize, ZoneId)>);
