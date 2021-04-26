use super::system_prelude::*;

#[derive(Default)]
pub struct UpdateRotate;

impl<'a> System<'a> for UpdateRotate {
    type SystemData = (ReadStorage<'a, Rotate>, WriteStorage<'a, Transform>);

    fn run(&mut self, (rotate_store, mut transform_store): Self::SystemData) {
        for (rotate, transform) in (&rotate_store, &mut transform_store).join()
        {
            transform.rotate_2d(rotate.step);
        }
    }
}
