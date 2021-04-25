use super::system_prelude::*;
use deathframe::physics::query;
use query::prelude::{FindQuery, Query};

#[derive(Default)]
pub struct HandleZoneSwitch;

impl<'a> System<'a> for HandleZoneSwitch {
    type SystemData = (
        WriteExpect<'a, ShouldLoadNextZone>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            mut should_load_next_zone,
            player_store,
            collider_store,
        ): Self::SystemData,
    ) {
        if !should_load_next_zone.0 {
            let should_switch_zone = {
                (&player_store, &collider_store).join().any(
                    |(_, player_collider)| {
                        let query_exp = {
                            use query::exp::prelude_variants::*;
                            And(vec![
                                IsState(Enter),
                                IsTag(CollisionTag::Portal),
                            ])
                        };
                        player_collider
                            .query::<FindQuery<CollisionTag>>()
                            .exp(&query_exp)
                            .run()
                            .is_some()
                    },
                )
            };

            if dbg!(should_switch_zone) {
                should_load_next_zone.0 = true;
            }
        }
    }
}
