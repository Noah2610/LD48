mod control_player;
mod handle_segment_loading;
mod update_on_lane;

pub mod prelude {
    pub use super::control_player::ControlPlayer;
    pub use super::handle_segment_loading::HandleSegmentLoading;
    pub use super::update_on_lane::UpdateOnLane;
    pub use deathframe::systems::prelude::*;
}

mod system_prelude {
    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
    pub use deathframe::core::geo::prelude::*;
    pub use deathframe::systems::system_prelude::*;
}
