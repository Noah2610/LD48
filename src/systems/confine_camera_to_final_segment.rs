use super::system_prelude::*;
use deathframe::core::geo::prelude::Rect;

#[derive(Default)]
pub struct ConfineCameraToFinalSegment;

impl<'a> System<'a> for ConfineCameraToFinalSegment {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, ZonesManager>,
        ReadStorage<'a, Camera>,
        WriteStorage<'a, Confined>,
        ReadStorage<'a, Segment>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Size>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut zones_manager,
            camera_store,
            mut confined_store,
            segment_store,
            transform_store,
            size_store,
        ): Self::SystemData,
    ) {
        if let Some((camera_entity, _, _)) =
            (&entities, &camera_store, !&confined_store).join().next()
        {
            if let Some((segment_transform, segment_size)) =
                (&segment_store, &transform_store, &size_store)
                    .join()
                    .find_map(|(segment, transform, size)| {
                        if segment.is_final_segment {
                            Some((transform, size))
                        } else {
                            None
                        }
                    })
            {
                zones_manager.lock_segment_loading();

                let segment_pos = segment_transform.translation();

                let _ = confined_store.insert(
                    camera_entity,
                    Confined::from(Rect {
                        top:    0.0,
                        bottom: segment_pos.y - segment_size.h * 0.5,
                        left:   0.0,
                        right:  segment_size.w,
                    }),
                );
            }
        }
    }
}
