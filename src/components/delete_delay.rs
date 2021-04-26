use super::component_prelude::*;
use climer::Timer;
use std::time::Duration;

#[derive(Component, Clone, Deserialize)]
#[storage(DenseVecStorage)]
pub struct DeleteDelay {
    pub delete_after_ms: u64,
    #[serde(skip)]
    timer:               Option<Timer>,
}

impl DeleteDelay {
    pub fn get_timer(&mut self) -> &mut Timer {
        let ms = self.delete_after_ms;
        self.timer.get_or_insert_with(|| {
            let mut timer =
                Timer::new(Some(Duration::from_millis(ms).into()), None);
            let _ = timer.start();
            timer
        })
    }
}
