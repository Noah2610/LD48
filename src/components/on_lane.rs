use super::component_prelude::*;

#[derive(Component, Deserialize, Clone)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct OnLane {
    pub switch_speed: f32,
    #[serde(skip)]
    pub current:      Option<usize>,
    #[serde(skip)]
    pub moving_dir:   Option<Dir>,
    #[serde(skip)]
    actions:          Vec<OnLaneAction>,
}

#[derive(Clone)]
pub enum Dir {
    Left,
    Right,
}

#[derive(Clone)]
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
