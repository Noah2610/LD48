use crate::settings::zones_settings::ZoneId;

#[derive(Default)]
pub struct SelectedZone(pub Option<(ZoneId, usize)>);
