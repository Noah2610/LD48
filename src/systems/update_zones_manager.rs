use super::system_prelude::*;

#[derive(Default)]
pub struct UpdateZonesManager;

impl<'a> System<'a> for UpdateZonesManager {
    type SystemData = (
        WriteExpect<'a, ZonesManager>,
        ReadExpect<'a, ZonesSettings>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            mut zones_manager,
            zones_settings,
            camera_store,
            collider_store,
        ): Self::SystemData,
    ) {
        for (_, collider) in (&camera_store, &collider_store).join() {
            let did_segment_leave = {
                use deathframe::physics::query::exp::prelude_variants::*;
                use deathframe::physics::query::prelude::{FindQuery, Query};

                collider
                    .query::<FindQuery<CollisionTag>>()
                    .exp(&And(vec![
                        IsTag(CollisionTag::Segment),
                        IsState(Leave),
                    ]))
                    .run()
                    .is_some()
            };

            if did_segment_leave {
                zones_manager.stage_next_segment(&zones_settings);
            }
        }
    }
}
