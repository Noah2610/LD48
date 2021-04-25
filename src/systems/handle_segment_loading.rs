use super::system_prelude::*;

#[derive(Default)]
pub struct HandleSegmentLoading;

impl<'a> System<'a> for HandleSegmentLoading {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, ZonesManager>,
        ReadExpect<'a, ZonesSettings>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Segment>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Size>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut zones_manager,
            zones_settings,
            camera_store,
            segment_store,
            transform_store,
            size_store,
        ): Self::SystemData,
    ) {
        if let Some((_, camera_transform, camera_size)) =
            (&camera_store, &transform_store, &size_store).join().next()
        {
            let camera_top =
                camera_transform.translation().y + camera_size.h * 0.5;

            for (segment_entity, _, segment_transform, segment_size) in
                (&entities, &segment_store, &transform_store, &size_store)
                    .join()
            {
                let segment_bot =
                    segment_transform.translation().y - segment_size.h * 0.5;

                if segment_bot > camera_top {
                    zones_manager.stage_next_segment(&zones_settings);
                    let _ = entities.delete(segment_entity);
                }
            }
        }
    }
}
