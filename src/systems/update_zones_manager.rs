use super::system_prelude::*;

#[derive(Default)]
pub struct UpdateZonesManager;

impl<'a> System<'a> for UpdateZonesManager {
    type SystemData = (
        WriteExpect<'a, ZonesManager>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (mut zones_manager, camera_store, collider_store): Self::SystemData,
    ) {
        for (_, collider) in (&camera_store, &collider_store).join() {
            let query = {
                use deathframe::physics::query::exp::prelude_variants::*;
                use deathframe::physics::query::prelude::{FindQuery, Query};

                collider
                    .query::<FindQuery<CollisionTag>>()
                    .exp(&IsState(Leave))
                    .run()
                    .is_some()
            };
        }
    }
}
