use super::system_prelude::*;
use deathframe::physics::query;
use query::prelude::{FindQuery, Query};

#[derive(Default)]
pub struct HandleObstacle;

impl<'a> System<'a> for HandleObstacle {
    type SystemData = (
        WriteExpect<'a, GameOver>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (mut game_over, player_store, collider_store): Self::SystemData,
    ) {
        if !game_over.0 {
            let is_game_over =
                (&player_store, &collider_store)
                    .join()
                    .any(|(_, collider)| {
                        let query_exp = {
                            use query::exp::prelude_variants::*;
                            And(vec![
                                IsState(Enter),
                                IsTag(CollisionTag::Obstacle),
                            ])
                        };
                        collider
                            .query::<FindQuery<CollisionTag>>()
                            .exp(&query_exp)
                            .run()
                            .is_some()
                    });

            if is_game_over {
                game_over.0 = true;
            }
        }
    }
}
