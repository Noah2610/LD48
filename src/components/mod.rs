pub mod belongs_to_segment;
pub mod camera;
pub mod object;
pub mod on_lane;
pub mod player;
pub mod segment;
pub mod tile;

pub mod prelude {
    pub use super::belongs_to_segment::BelongsToSegment;
    pub use super::camera::Camera;
    pub use super::object::Object;
    pub use super::on_lane::OnLane;
    pub use super::player::Player;
    pub use super::segment::Segment;
    pub use super::tile::Tile;
    pub use deathframe::components::prelude::*;
}

mod component_prelude {
    pub use super::prelude::*;
    pub use deathframe::components::component_prelude::*;
}
