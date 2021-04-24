pub mod object;
pub mod on_lane;
pub mod player;
pub mod tile;

pub mod prelude {
    pub use super::object::Object;
    pub use super::on_lane::OnLane;
    pub use super::player::Player;
    pub use super::tile::Tile;
    pub use deathframe::components::prelude::*;
}

mod component_prelude {
    pub use super::prelude::*;
    pub use deathframe::components::component_prelude::*;
}
