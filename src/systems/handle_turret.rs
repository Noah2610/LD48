use super::system_prelude::*;

#[derive(Default)]
pub struct HandleTurret;

impl<'a> System<'a> for HandleTurret {
    type SystemData = (WriteStorage<'a, Turret>, ReadStorage<'a, Unloaded>);

    fn run(&mut self, (mut turret_store, unloaded_store): Self::SystemData) {
        for (turret, _) in (&mut turret_store, !&unloaded_store).join() {
            let mut timer = turret.get_timer();
            let _ = timer.update();
            if timer.state.is_finished() {
                let _ = timer.start();
            }
        }
    }
}
