use super::system_prelude::*;
use crate::components::on_lane::OnLaneAction;

#[derive(Default)]
pub struct ControlPlayer;

impl<'a> System<'a> for ControlPlayer {
    type SystemData = (
        ReadExpect<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, OnLane>,
    );

    fn run(
        &mut self,
        (input_manager, player_store, mut on_lane_store): Self::SystemData,
    ) {
        let on_lane_action_opt =
            if input_manager.is_down(IngameAction::NextLane) {
                Some(OnLaneAction::NextLane)
            } else if input_manager.is_down(IngameAction::PrevLane) {
                Some(OnLaneAction::PrevLane)
            } else {
                None
            };

        if let Some(on_lane_action) = on_lane_action_opt {
            if let Some((_, on_lane)) =
                (&player_store, &mut on_lane_store).join().next()
            {
                on_lane.add_action(on_lane_action);
            }
        }
    }
}
