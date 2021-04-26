use super::system_prelude::*;
use deathframe::physics::query;
use query::prelude::{FindQuery, Query};

#[derive(Default)]
pub struct HandleObstacle;

impl<'a> System<'a> for HandleObstacle {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameOver>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Obstacle>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut game_over,
            player_store,
            obstacle_store,
            collider_store,
        ): Self::SystemData,
    ) {
        if !game_over.0 {
            let is_game_over =
                (&player_store, &collider_store)
                    .join()
                    .any(|(_, collider)| {
                        let query_exp = {
                            use query::exp::prelude_variants::*;
                            And(vec![
                                IsState(Steady),
                                IsTag(CollisionTag::Obstacle),
                            ])
                        };
                        let collision = collider
                            .query::<FindQuery<CollisionTag>>()
                            .exp(&query_exp)
                            .run();
                        if let Some(collision) = collision {
                            (&entities, &obstacle_store).join().any(
                                |(obstacle_entity, _)| {
                                    obstacle_entity.id() == collision.id
                                },
                            )
                        } else {
                            false
                        }
                    });

            if is_game_over {
                game_over.0 = true;
            }
        }
    }
}
