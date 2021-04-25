use super::system_prelude::*;
use deathframe::core::geo::prelude::Rect;

#[derive(Default)]
pub struct HandleZoneSwitch;

impl<'a> System<'a> for HandleZoneSwitch {
    type SystemData = (WriteExpect<'a, ZonesManager>,);

    fn run(&mut self, (mut zones_manager,): Self::SystemData) {
    }
}
