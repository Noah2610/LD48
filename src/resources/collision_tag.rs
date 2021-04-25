use deathframe::physics::CollisionTag as CTag;

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum CollisionTag {
    Player,
    Solid,
    Portal,
    Obstacle,
}

impl CTag for CollisionTag {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (CollisionTag::Player, CollisionTag::Solid) => true,
            (CollisionTag::Player, CollisionTag::Portal) => true,
            (CollisionTag::Player, CollisionTag::Obstacle) => true,
            _ => false,
        }
    }
}

pub type SolidTag = CollisionTag;
