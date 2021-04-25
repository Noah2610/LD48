use deathframe::amethyst::ecs::Entity;
use std::collections::hash_set::HashSet;

#[derive(Default)]
pub struct EntitiesToDelete {
    pub to_delete: HashSet<Entity>,
}

impl EntitiesToDelete {
    pub fn stage(&mut self, entity: Entity) {
        self.to_delete.insert(entity);
    }
}
