pub mod prelude {
    pub use super::animation_key::AnimationKey;
    pub use super::collision_tag::{CollisionTag, SolidTag};
    pub use super::dispatcher_id::DispatcherId;
    pub use super::lanes::{Lane, Lanes};
    pub use super::song_key::SongKey;
    pub use super::sound_key::SoundKey;
    pub use super::zone_height::ZoneHeight;
    pub use super::zones_manager::ZonesManager;
    pub use deathframe::resources::prelude::*;
}

mod animation_key;
mod collision_tag;
mod dispatcher_id;
mod lanes;
mod song_key;
mod sound_key;
mod zone_height;
mod zones_manager;
