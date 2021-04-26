use super::component_prelude::*;
use crate::level_loader::ObjectType;
use climer::Timer;
use std::time::Duration;

#[derive(Component, Clone, Deserialize)]
#[storage(DenseVecStorage)]
pub struct Turret {
    pub shot_object_type:      ObjectType,
    pub shot_interval_ms:      u64,
    #[serde(default)]
    pub shot_initial_delay_ms: u64,
    #[serde(default)]
    pub shot_offset:           (f32, f32),
    #[serde(skip)]
    shot_timer:                Option<Timer>,
}

impl Turret {
    pub fn get_timer(&mut self) -> &mut Timer {
        let interval_ms = self.shot_interval_ms;
        let initial_delay_ms = self.shot_initial_delay_ms;
        self.shot_timer.get_or_insert_with(|| {
            let mut timer = Timer::new(
                Some(Duration::from_millis(interval_ms).into()),
                None,
            );
            timer
                .set_start_time(Duration::from_millis(initial_delay_ms).into());
            let _ = timer.start();
            timer
        })
    }
}
