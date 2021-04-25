use super::system_prelude::*;

#[derive(Default)]
pub struct HandleParentDelete;

impl<'a> System<'a> for HandleParentDelete {
    type SystemData = (Entities<'a>, ReadStorage<'a, ParentDelete>);

    fn run(&mut self, (entities, parent_delete_store): Self::SystemData) {
        for (child, parent_delete) in (&entities, &parent_delete_store).join() {
            if !entities.is_alive(parent_delete.0) {
                let _ = entities.delete(child);
            }
        }
    }
}
