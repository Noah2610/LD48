use crate::settings::prelude::LanesSettings;

pub struct Lanes {
    pub lanes: Vec<Lane>,
}

pub struct Lane {
    pub x: f32,
}

impl From<(&LanesSettings, f32)> for Lanes {
    fn from((settings, level_width): (&LanesSettings, f32)) -> Self {
        let center_x = level_width * 0.5;
        let total_lanes_width = settings.spacing * settings.count as f32;
        let half_lanes_width = total_lanes_width * 0.5;
        let half_lane_width = settings.spacing * 0.5;

        let lanes = (0 .. settings.count)
            .into_iter()
            .map(|i| Lane {
                x: center_x + (i as f32 * settings.spacing) - half_lanes_width
                    + half_lane_width,
            })
            .collect();

        Self { lanes }
    }
}
