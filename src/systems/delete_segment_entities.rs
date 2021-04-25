use super::system_prelude::*;

#[derive(Default)]
pub struct DeleteSegmentEntities;

impl<'a> System<'a> for DeleteSegmentEntities {
    type SystemData = (
        Entities<'a>,
        Write<'a, SegmentsToDelete>,
        ReadStorage<'a, Segment>,
    );

    fn run(
        &mut self,
        (entities, mut segments_to_delete, segment_store): Self::SystemData,
    ) {
        for (entity, segment) in (&entities, &segment_store).join() {
            if segments_to_delete.to_delete.contains(&segment.0) {
                let _ = entities.delete(entity);
            }
        }

        segments_to_delete.to_delete.clear();
    }
}
