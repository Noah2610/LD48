use super::menu_prelude::*;
use super::state_prelude::*;
use crate::components::prelude::Size;
use crate::input::prelude::{MenuAction, MenuBindings};
use crate::level_loader::objects::build_camera;
use crate::level_loader::{build_level, load_level};

#[derive(Default)]
pub struct Cutscene {
    ui_data: UiData,
}

impl Cutscene {
    fn start<'a, 'b>(&mut self, data: &mut StateData<GameData<'a, 'b>>) {
        self.create_ui(data, resource("ui/cutscene.ron").to_str().unwrap());
    }

    fn stop<'a, 'b>(&mut self, data: &mut StateData<GameData<'a, 'b>>) {
        self.delete_ui(data);
    }

    fn handle_input<'a, 'b>(
        &mut self,
        input_manager: &InputManager<MenuBindings>,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        if input_manager.is_down(MenuAction::Start) {
            Some(Trans::Switch(Box::new(MainMenu::default())))
        } else {
            None
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Cutscene {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.start(&mut data);

        let level_size = {
            let level_data = load_level(resource("levels/cutscene.json"))
                .expect("Couldn't load cutscene level");
            let level_size =
                Size::new(level_data.level.size.w, level_data.level.size.h);
            build_level(data.world, level_data)
                .expect("Couldn't build cutscene level");
            level_size
        };

        {
            build_camera(
                data.world,
                None,
                level_size.clone(),
                Some(level_size),
            )
            .expect("Couldn't load cutscene camera");
        };
    }

    fn on_resume(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.start(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.stop(&mut data);
        data.world.delete_all();
    }

    fn on_pause(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.stop(&mut data);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update_core(data.world);

        if let Some(trans) = self.handle_input(
            &*data.world.read_resource::<InputManager<MenuBindings>>(),
        ) {
            return trans;
        }

        {
            use crate::components::prelude::{AnimationsContainer, Cutscene};
            use deathframe::amethyst::ecs::{Join, ReadStorage};

            let is_cutscene_finished = data.world.exec(
                |(cutscene_store, animations_store): (
                    ReadStorage<Cutscene>,
                    ReadStorage<AnimationsContainer<AnimationKey>>,
                )| {
                    (&cutscene_store, &animations_store).join().all(
                        |(_, animations)| {
                            animations.last_finished_animation().is_some()
                        },
                    )
                },
            );

            if is_cutscene_finished {
                return Trans::Switch(Box::new(MainMenu::default()));
            }
        }

        Trans::None
    }

    fn fixed_update(
        &mut self,
        mut data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        if let Some(trans) = self.update_ui_events(&mut data) {
            trans
        } else {
            Trans::None
        }
    }
}

impl<'a, 'b> Menu<GameData<'a, 'b>, StateEvent> for Cutscene {
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
