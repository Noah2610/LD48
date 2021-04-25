use super::system_prelude::*;
use deathframe::core::geo::prelude::Rect;

#[derive(Default)]
pub struct HandleZoneSwitch;

impl<'a> System<'a> for HandleZoneSwitch {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, ZonesManager>,
        ReadExpect<'a, ZonesSettings>,
        ReadExpect<'a, ZoneSize>,
        ReadStorage<'a, Camera>,
        WriteStorage<'a, Confined>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut zones_manager,
            zones_settings,
            zone_size,
            camera_store,
            mut confined_store,
        ): Self::SystemData,
    ) {
        if zones_manager.is_final_segment_loaded(&zones_settings) {
            zones_manager.lock_segment_loading();

            if let Some((camera_entity, _)) =
                (&entities, &camera_store).join().next()
            {
                let _ = confined_store.insert(
                    camera_entity,
                    Confined::from(Rect {
                        top:    0.0,
                        bottom: -zone_size.height,
                        left:   0.0,
                        right:  zone_size.width,
                    }),
                );
            }
        }
    }
}
