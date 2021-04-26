use super::system_prelude::*;

#[derive(Default)]
pub struct HandleTurret;

impl<'a> System<'a> for HandleTurret {
    type SystemData = (
        WriteExpect<'a, ObjectSpawner>,
        WriteExpect<'a, SoundPlayer<SoundKey>>,
        WriteStorage<'a, Turret>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            mut object_spawner,
            mut sound_player,
            mut turret_store,
            transform_store,
            unloaded_store,
        ): Self::SystemData,
    ) {
        for (turret, turret_transform, _) in
            (&mut turret_store, &transform_store, !&unloaded_store).join()
        {
            let timer = turret.get_timer();
            let _ = timer.update();
            if timer.state.is_finished() {
                let _ = timer.start();
                let pos = {
                    let trans = turret_transform.translation();
                    (trans.x, trans.y, trans.z + 1.0)
                };
                object_spawner.spawn(ObjectToSpawn {
                    object_type: turret.shot_object_type.clone(),
                    pos,
                    size: None,
                });
                sound_player.add_action(SoundAction::Play(SoundKey::Shoot));
            }
        }
    }
}
