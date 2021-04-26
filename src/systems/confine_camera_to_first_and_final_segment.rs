use super::system_prelude::*;
use deathframe::core::geo::prelude::Rect;

// stupid
const FIRST_SEGMENT_CONFINE_BOTTOM: f32 = 100_000.0;

#[derive(Default)]
pub struct ConfineCameraToFirstAndFinalSegment;

impl<'a> System<'a> for ConfineCameraToFirstAndFinalSegment {
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
        if let Some(camera_entity) = (&entities, &camera_store)
            .join()
            .map(|(entity, _)| entity)
            .next()
        {
            let mut first_segment_opt = None;
            let mut final_segment_opt = None;

            for (segment, transform, size) in
                (&segment_store, &transform_store, &size_store).join()
            {
                if first_segment_opt.is_some() && final_segment_opt.is_some() {
                    break;
                }
                if segment.is_first_segment && first_segment_opt.is_none() {
                    first_segment_opt = Some((transform, size));
                }
                if segment.is_final_segment && final_segment_opt.is_none() {
                    final_segment_opt = Some((transform, size));
                    first_segment_opt = None;
                    break;
                }
            }

            if let Some((final_segment_transform, final_segment_size)) =
                final_segment_opt
            {
                zones_manager.lock_segment_loading();

                let segment_pos = final_segment_transform.translation();

                let _ = confined_store.insert(
                    camera_entity,
                    Confined::from(Rect {
                        top:    0.0,
                        bottom: segment_pos.y - final_segment_size.h * 0.5,
                        left:   0.0,
                        right:  final_segment_size.w,
                    }),
                );
            } else {
                if let Some((_, first_segment_size)) = first_segment_opt {
                    let _ = confined_store.insert(
                        camera_entity,
                        Confined::from(Rect {
                            top:    0.0,
                            bottom: -FIRST_SEGMENT_CONFINE_BOTTOM,
                            left:   0.0,
                            right:  first_segment_size.w,
                        }),
                    );
                } else {
                    confined_store.remove(camera_entity);
                }
            }
        }
    }
}
