// resources/settings/zones

use deathframe::core::components::prelude::Merge;
use replace_with::replace_with_or_abort;
use std::collections::HashMap;

pub type ZoneId = String;

#[derive(Deserialize, Default)]
pub struct ZonesSettings {
    pub config: ZonesConfig,
    pub zones:  HashMap<ZoneId, ZoneSettings>,
}

#[derive(Deserialize, Default)]
pub struct ZonesConfig {}

#[derive(Deserialize)]
pub struct ZoneSettings {}

impl Merge for ZonesSettings {
    fn merge(&mut self, other: Self) {
        let ZonesSettings {
            config: other_config,
            zones: mut other_zones,
        } = other;
        replace_with_or_abort(self, |self_| ZonesSettings {
            config: self_.config.merged(other_config),
            zones:  self_
                .zones
                .into_iter()
                .map(|(zone_id, zone_settings)| {
                    let merged_zone_settings = if let Some(other_zone) =
                        other_zones.remove(&zone_id)
                    {
                        zone_settings.merged(other_zone)
                    } else {
                        zone_settings
                    };
                    (zone_id, merged_zone_settings)
                })
                .collect(),
        });
    }
}

impl Merge for ZonesConfig {
    fn merge(&mut self, other: Self) {
        // TODO
    }
}

impl Merge for ZoneSettings {
    fn merge(&mut self, other: Self) {
        // TODO
    }
}
