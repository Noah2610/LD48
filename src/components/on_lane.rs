use super::component_prelude::*;

#[derive(Component, Deserialize)]
#[storage(VecStorage)]
pub struct OnLane {
    pub current:      usize,
    pub switch_speed: f32,
    #[serde(skip)]
    pub moving_dir:   Option<Dir>,
    #[serde(skip)]
    actions:          Vec<OnLaneAction>,
}

pub enum Dir {
    Left,
    Right,
}

impl OnLane {
    pub fn next_lane(&mut self) {
        self.actions.push(OnLaneAction::NextLane);
    }

    pub fn prev_lane(&mut self) {
        self.actions.push(OnLaneAction::PrevLane);
    }
}

pub enum OnLaneAction {
    NextLane,
    PrevLane,
}

impl ActionQueue for OnLane {
    type Action = OnLaneAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
