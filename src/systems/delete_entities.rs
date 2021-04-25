use super::system_prelude::*;

#[derive(Default)]
pub struct DeleteEntities;

impl<'a> System<'a> for DeleteEntities {
    type SystemData = (
        Entities<'a>,
        Write<'a, EntitiesToDelete>,
        ReadStorage<'a, Segment>,
    );

    fn run(
        &mut self,
        (entities, mut entities_to_delete, segment_store): Self::SystemData,
    ) {
        for (entity, segment) in (&entities, &segment_store).join() {
            if entities_to_delete.to_delete.contains(&entity) {
                let _ = entities.delete(entity);
            }
        }

        entities_to_delete.to_delete.clear();
    }
}
