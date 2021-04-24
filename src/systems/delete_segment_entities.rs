use super::system_prelude::*;

#[derive(Default)]
pub struct DeleteSegmentEntities;

impl<'a> System<'a> for DeleteSegmentEntities {
    type SystemData = (
        Entities<'a>,
        Write<'a, SegmentsToDelete>,
        ReadStorage<'a, BelongsToSegment>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut segments_to_delete,
            belongs_to_segment_store,
            camera_store,
            transform_store,
        ): Self::SystemData,
    ) {
        if let Some(camera_pos) = (&camera_store, &transform_store)
            .join()
            .next()
            .map(|(_, transform)| transform.translation())
        {
            for (entity, belongs_to_segment, transform) in
                (&entities, &belongs_to_segment_store, &transform_store).join()
            {
                let pos = transform.translation();

                if pos.y > camera_pos.y
                    && segments_to_delete
                        .to_delete
                        .contains(&belongs_to_segment.0)
                {
                    let _ = entities.delete(entity);
                }
            }
        }

        segments_to_delete.to_delete.clear();
    }
}
