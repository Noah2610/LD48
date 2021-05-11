use super::system_prelude::*;

#[derive(Default)]
pub struct HandleZoneSelect;

impl<'a> System<'a> for HandleZoneSelect {
    type SystemData = (
        ReadExpect<'a, InputManager<MenuBindings>>,
        Write<'a, SelectedZone>,
    );

    fn run(&mut self, (input_manager, mut selected_zone): Self::SystemData) {
    }
}
