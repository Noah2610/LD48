use super::state_prelude::*;
use crate::components::prelude::{Size, Transform};
use crate::level_loader::objects::{build_camera, build_player};
use crate::level_loader::{build_level, load_level};
use crate::resource;

#[derive(Default)]
pub struct Ingame;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        let level_data =
            load_level(resource("levels/zones/dev_0/00.json")).unwrap();
        let level_size =
            Size::new(level_data.level.size.w, level_data.level.size.h);
        build_level(data.world, level_data).unwrap();

        {
            let lanes = Lanes::from((
                &*data.world.read_resource::<LanesSettings>(),
                &level_size,
            ));
            data.world.insert(lanes);
        }

        {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                level_size.w * 0.5,
                level_size.h,
                2.0,
            );
            let size = Size::new(32.0, 32.0);
            let player = build_player(data.world, transform, size);
            let _ = build_camera(data.world, player, level_size);
        }
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();

        Trans::None
    }

    // fn fixed_update(
    //     &mut self,
    //     data: StateData<GameData<'a, 'b>>,
    // ) -> Trans<GameData<'a, 'b>, StateEvent> {
    //     let zones_manager = data.world.write_resource::<ZonesManager>();
    //     zones_manager.update(data.world);

    //     Trans::None
    // }
}
