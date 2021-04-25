use super::state_prelude::*;
use crate::components::prelude::{Size, Transform};
use crate::level_loader::build_level;
use crate::level_loader::objects::{build_camera, build_player};

const SEGMENT_WIDTH: f32 = 96.0;

#[derive(Default)]
pub struct Ingame;

impl Ingame {
    fn start(&mut self, world: &mut World) {
        world.delete_all();

        world.write_resource::<ZoneSize>().reset();

        {
            let mut transform = Transform::default();
            transform.set_translation_xyz(0.0, 0.0, 2.0);
            let size = Size::new(32.0, 32.0);
            let player = build_player(world, transform, size);
            let _ = build_camera(world, player, SEGMENT_WIDTH);
        }

        {
            use deathframe::amethyst::ecs::{ReadExpect, WriteExpect};

            world.exec(
                |(mut zones_manager, settings): (
                    WriteExpect<ZonesManager>,
                    ReadExpect<ZonesSettings>,
                )| {
                    zones_manager.stage_initial_segments(&settings);
                },
            );
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        // TODO!!!
        // let level_data =
        //     load_level(resource("levels/zones/dev_0/00.json")).unwrap();
        // let level_size =
        //     Size::new(level_data.level.size.w, level_data.level.size.h);
        // build_level(data.world, level_data).unwrap();

        {
            let lanes = Lanes::from((
                &*data.world.read_resource::<LanesSettings>(),
                SEGMENT_WIDTH,
            ));
            data.world.insert(lanes);
        }

        {
            use deathframe::amethyst::ecs::{ReadExpect, WriteExpect};

            data.world.exec(
                |(mut zones_manager, settings): (
                    WriteExpect<ZonesManager>,
                    ReadExpect<ZonesSettings>,
                )| {
                    zones_manager.stage_next_zone(&settings);
                },
            );
        }

        self.start(data.world);
    }

    fn on_resume(&mut self, data: StateData<GameData<'a, 'b>>) {
        self.start(data.world);
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
        for (segment_id, level) in levels_to_load {
            build_level(data.world, level, segment_id).unwrap();
        }

        let mut should_load_next_zone =
            data.world.write_resource::<ShouldLoadNextZone>();
        if should_load_next_zone.0 {
            should_load_next_zone.0 = false;
            return Trans::Push(Box::new(ZoneTransition::default()));
        }

        Trans::None
    }
}
