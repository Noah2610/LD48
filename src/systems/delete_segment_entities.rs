use super::system_prelude::*;

#[derive(Default)]
pub struct DeleteSegmentEntities;

impl<'a> System<'a> for DeleteSegmentEntities {
    type SystemData = (
        Entities<'a>,
        Write<'a, SegmentsToDelete>,
        ReadStorage<'a, BelongsToSegment>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut segments_to_delete,
            belongs_to_segment_store,
        ): Self::SystemData,
    ) {
        for (entity, belongs_to_segment) in
            (&entities, &belongs_to_segment_store).join()
        {
            if segments_to_delete.to_delete.contains(&belongs_to_segment.0) {
                let _ = entities.delete(entity);
            }
        }

        segments_to_delete.to_delete.clear();
    }
}
