use super::system_prelude::*;

#[derive(Default)]
pub struct UpdateZonesManager;

impl<'a> System<'a> for UpdateZonesManager {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, ZonesManager>,
        ReadExpect<'a, ZonesSettings>,
        Write<'a, SegmentsToDelete>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Collider<CollisionTag>>,
        ReadStorage<'a, Segment>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut zones_manager,
            zones_settings,
            mut segments_to_delete,
            camera_store,
            collider_store,
            segment_store,
        ): Self::SystemData,
    ) {
        for (_, collider) in (&camera_store, &collider_store).join() {
            let segment_leave_id_opt = {
                use deathframe::physics::query::exp::prelude_variants::*;
                use deathframe::physics::query::prelude::{FindQuery, Query};

                let query_exp =
                    And(vec![IsTag(CollisionTag::Segment), IsState(Leave)]);

                collider
                    .query::<FindQuery<CollisionTag>>()
                    .exp(&query_exp)
                    .run()
                    .map(|data| data.id)
            };

            if let Some(segment_id) = segment_leave_id_opt {
                zones_manager.stage_next_segment(&zones_settings);
                if let Some(segment) =
                    segment_store.get(entities.entity(segment_id))
                {
                    segments_to_delete.stage(segment.0.clone());
                }
            }
        }
    }
}
