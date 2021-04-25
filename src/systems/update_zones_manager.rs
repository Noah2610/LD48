use super::system_prelude::*;

#[derive(Default)]
pub struct UpdateZonesManager;

impl<'a> System<'a> for UpdateZonesManager {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, ZonesManager>,
        ReadExpect<'a, ZonesSettings>,
        Write<'a, EntitiesToDelete>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut zones_manager,
            zones_settings,
            mut entities_to_delete,
            camera_store,
            collider_store,
        ): Self::SystemData,
    ) {
        for (_, collider) in (&camera_store, &collider_store).join() {
            let segment_leave_id_opt = {
                use deathframe::physics::query::exp::prelude_variants::*;
                use deathframe::physics::query::prelude::{
                    FilterQuery,
                    FindQuery,
                    Query,
                };

                let query_exp =
                    And(vec![IsTag(CollisionTag::Segment), IsState(Leave)]);

                // let query_debug = IsTag(CollisionTag::Segment);
                // dbg!(collider
                //     .query::<FilterQuery<CollisionTag>>()
                //     .exp(&query_debug)
                //     .run()
                //     .len());

                collider
                    .query::<FindQuery<CollisionTag>>()
                    .exp(&query_exp)
                    .run()
                    .map(|data| data.id)
            };

            if let Some(entity_id) = segment_leave_id_opt {
                println!("Load next segment");
                zones_manager.stage_next_segment(&zones_settings);
                entities_to_delete.stage(entities.entity(entity_id));
            }
        }
    }
}
