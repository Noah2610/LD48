use super::system_prelude::*;
use deathframe::physics::query;
use query::prelude::{FilterQuery, Query};

#[derive(Default)]
pub struct HandleCoinCollection;

impl<'a> System<'a> for HandleCoinCollection {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Score>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (entities, mut score, player_store, collider_store): Self::SystemData,
    ) {
        let query_exp = {
            use query::exp::prelude_variants::*;
            And(vec![IsState(Enter), IsTag(CollisionTag::Coin)])
        };

        for (_, collider) in (&player_store, &collider_store).join() {
            let collisions = collider
                .query::<FilterQuery<CollisionTag>>()
                .exp(&query_exp)
                .run();
            for collision in collisions {
                let _ = entities.delete(entities.entity(collision.id));
                score.coins += 1;
            }
        }
    }
}
