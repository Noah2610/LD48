use super::system_prelude::*;
use crate::components::on_lane::{Dir, OnLaneAction};

#[derive(Default)]
pub struct UpdateOnLane;

impl<'a> System<'a> for UpdateOnLane {
    type SystemData = (
        ReadExpect<'a, Lanes>,
        WriteStorage<'a, OnLane>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
    );

    fn run(
        &mut self,
        (
            lanes,
            mut on_lane_store,
            mut transform_store,
            mut velocity_store,
            mut animations_container_store,
        ): Self::SystemData,
    ) {
        let lanes_count = lanes.lanes.len();

        for (on_lane, transform, velocity, animations_opt) in (
            &mut on_lane_store,
            &mut transform_store,
            &mut velocity_store,
            (&mut animations_container_store).maybe(),
        )
            .join()
        {
            let mut next_lane = on_lane.current;

            for action in on_lane.drain_actions() {
                match action {
                    OnLaneAction::NextLane => {
                        next_lane = (next_lane + 1).min(lanes_count - 1)
                    }
                    OnLaneAction::PrevLane => {
                        next_lane = next_lane.checked_sub(1).unwrap_or(0)
                    }
                }
            }

            let pos_x = transform.translation().x;
            let target_lane_x = lanes.lanes[next_lane].x;

            let should_switch_lane = next_lane != on_lane.current;
            if should_switch_lane {
                let pos_diff = target_lane_x - pos_x;
                let is_moving_right = pos_diff > 0.0;

                let moving_dir = if is_moving_right {
                    Dir::Right
                } else {
                    Dir::Left
                };

                on_lane.current = next_lane;
                on_lane.moving_dir = Some(moving_dir);

                velocity.x = on_lane.switch_speed * pos_diff.signum();

                if let Some(animations) = animations_opt {
                    if is_moving_right {
                        let _ = animations.push(AnimationKey::NextLane);
                    } else {
                        let _ = animations.push(AnimationKey::PrevLane);
                    }
                }
            }

            if let Some(moving_dir) = &on_lane.moving_dir {
                let stop_moving = match moving_dir {
                    Dir::Right => pos_x >= target_lane_x,
                    Dir::Left => pos_x <= target_lane_x,
                };

                if stop_moving {
                    on_lane.moving_dir = None;
                    velocity.x = 0.0;
                    transform.set_translation_x(target_lane_x);
                }
            }
        }
    }
}
