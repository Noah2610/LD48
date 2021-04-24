use super::state_prelude::*;
use crate::components::prelude::Size;
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

        let lanes = Lanes::from((
            &*data.world.read_resource::<LanesSettings>(),
            &level_size,
        ));
        data.world.insert(lanes);
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
