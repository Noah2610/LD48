use crate::settings::prelude::LanesSettings;

pub struct Lanes {
    pub lanes: Vec<Lane>,
}

pub struct Lane {
    pub x: f32,
}

impl From<&LanesSettings> for Lanes {
    fn from(settings: &LanesSettings) -> Self {
        let total_lanes_width = settings.spacing * settings.count as f32;
        let half_lanes_width = total_lanes_width * 0.5;

        let lanes = (0..settings.count)
            .into_iter()
            .map(|i| Lane {
                x: (i as f32 * settings.spacing) - half_lanes_width,
            })
            .collect();

        Self { lanes }
    }
}
