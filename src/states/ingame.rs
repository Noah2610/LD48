use super::menu_prelude::*;
use super::state_prelude::*;
use crate::components::prelude::{Size, Transform};
use crate::level_loader::build_level;
use crate::level_loader::objects::{build_camera, build_object, build_player};

const SEGMENT_WIDTH: f32 = 128.0;

#[derive(Default)]
pub struct Ingame {
    ui_data: UiData,
}

impl Ingame {
    fn start<'a, 'b>(&mut self, mut data: &mut StateData<GameData<'a, 'b>>) {
        data.world.delete_all();

        self.create_ui(&mut data, resource("ui/ingame.ron").to_str().unwrap());

        data.world.insert(ObjectSpawner::default());
        data.world.write_resource::<ZoneSize>().reset();

        {
            let mut transform = Transform::default();
            transform.set_translation_xyz(0.0, 64.0, 2.0);
            let size = Size::new(32.0, 32.0);
            let player = build_player(data.world, transform, size);
            let _ = build_camera(data.world, player, SEGMENT_WIDTH);
        }

        {
            use deathframe::amethyst::ecs::{ReadExpect, WriteExpect};

            data.world.exec(
                |(mut zones_manager, settings, mut songs): (
                    WriteExpect<ZonesManager>,
                    ReadExpect<ZonesSettings>,
                    WriteExpect<Songs<SongKey>>,
                )| {
                    zones_manager.stage_initial_segments(&settings);
                    songs.stop_all();
                    if let Some(song_key) =
                        zones_manager.get_current_song(&settings)
                    {
                        songs.play(song_key);
                    }
                },
            );
        }
    }

    fn stop<'a, 'b>(&mut self, mut data: &mut StateData<GameData<'a, 'b>>) {
        self.delete_ui(&mut data);
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        data.world.insert(ZonesManager::default());
        data.world.insert(ZoneSize::default());
        data.world.insert(ShouldLoadNextZone::default());
        data.world.insert(GameOver::default());
        data.world.insert(Score::default());

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

        self.start(&mut data);
    }

    fn on_resume(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.start(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.stop(&mut data);
    }

    fn on_pause(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.stop(&mut data);
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

        {
            let mut should_load_next_zone =
                data.world.write_resource::<ShouldLoadNextZone>();
            if should_load_next_zone.0 {
                should_load_next_zone.0 = false;
                return Trans::Push(Box::new(ZoneTransition::default()));
            }
        }

        {
            let mut game_over = data.world.write_resource::<GameOver>();
            if game_over.0 {
                game_over.0 = false;
                return Trans::Pop;
            }
        }

        {
            let objects_to_spawn = data
                .world
                .write_resource::<ObjectSpawner>()
                .objects_to_spawn();
            if !objects_to_spawn.is_empty() {
                for object in objects_to_spawn {
                    let transform = {
                        let mut transform = Transform::default();
                        transform.set_translation_xyz(
                            object.pos.0,
                            object.pos.1,
                            object.pos.2,
                        );
                        transform
                    };
                    if let Some(entity_builder) = build_object(
                        data.world,
                        object.object_type,
                        transform,
                        object.size.map(Into::into),
                    ) {
                        entity_builder.build();
                    }
                }
            }
        }

        Trans::None
    }
}

impl<'a, 'b> Menu<GameData<'a, 'b>, StateEvent> for Ingame {
    fn event_triggered(
        &mut self,
        _data: &mut StateData<GameData<'a, 'b>>,
        event_name: String,
        event: UiEvent,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        if let UiEventType::ClickStop = event.event_type {
            match event_name.as_str() {
                _ => None,
            }
        } else {
            None
        }
    }

    fn ui_data(&self) -> &UiData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UiData {
        &mut self.ui_data
    }
}
