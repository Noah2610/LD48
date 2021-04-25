use super::system_prelude::*;

#[derive(Default)]
pub struct DeleteEntities;

impl<'a> System<'a> for DeleteEntities {
    type SystemData = (Entities<'a>, Write<'a, EntitiesToDelete>);

    fn run(&mut self, (entities, mut entities_to_delete): Self::SystemData) {
        for entity in entities_to_delete.to_delete.drain() {
            let _ = entities.delete(entity);
        }
    }
}
