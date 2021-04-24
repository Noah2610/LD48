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
        // build_level(data.world, level_data).unwrap();

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

        {
            use deathframe::amethyst::ecs::{ReadExpect, WriteExpect};

            data.world.exec(
                |(mut zones_manager, settings): (
                    WriteExpect<ZonesManager>,
                    ReadExpect<ZonesSettings>,
                )| {
                    zones_manager.set_zone("dev_0".to_string());
                    zones_manager.stage_next_segment(&settings);
                },
            );
        }
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();

        Trans::None
    }

    fn fixed_update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        let levels_to_load =
            data.world.write_resource::<ZonesManager>().levels_to_load();
        for level in levels_to_load {
            // let _ = data
            //     .world
            //     .write_resource::<Option<Lanes>>()
            //     .get_or_insert_with(|| {
            //         let lanes = Lanes::from((
            //             &*data.world.read_resource::<LanesSettings>(),
            //             &Size::new(level.level.size.w, level.level.size.h),
            //         ));
            //         lanes
            //     });

            build_level(data.world, level).unwrap();
        }

        Trans::None
    }
}
