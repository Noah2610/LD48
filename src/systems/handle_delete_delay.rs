use super::system_prelude::*;

#[derive(Default)]
pub struct HandleDeleteDelay;

impl<'a> System<'a> for HandleDeleteDelay {
    type SystemData = (Entities<'a>, WriteStorage<'a, DeleteDelay>);

    fn run(&mut self, (entities, mut delete_delay_store): Self::SystemData) {
        for (entity, delete_delay) in
            (&entities, &mut delete_delay_store).join()
        {
            let timer = delete_delay.get_timer();
            let _ = timer.update();
            if timer.state.is_finished() {
                let _ = entities.delete(entity);
            }
        }
    }
}
