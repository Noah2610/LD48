use crate::components::prelude::{Hitbox, Size};
use amethyst::ecs::{Builder, EntityBuilder};
use deathframe::amethyst;
use deathframe::core::geo::prelude::Rect;

#[derive(Deserialize, Clone)]
pub enum HitboxConfig {
    Size,
    Custom(Hitbox),
    SizeOffset(Rect),
}

impl HitboxConfig {
    pub fn add_hitbox_to_entity<'a>(
        &self,
        entity_builder: EntityBuilder<'a>,
        size_opt: Option<&Size>,
    ) -> EntityBuilder<'a> {
        match self {
            HitboxConfig::Size => {
                if let Some(size) = size_opt {
                    entity_builder.with(Hitbox::from(size))
                } else {
                    panic!("HitboxConfig::Size entity doesn't have a Size");
                }
            }
            HitboxConfig::Custom(hitbox) => entity_builder.with(hitbox.clone()),
            HitboxConfig::SizeOffset(padding) => {
                if let Some(size) = size_opt {
                    let mut rect = Rect::from(size);
                    rect.top += padding.top;
                    rect.bottom += padding.bottom;
                    rect.left += padding.left;
                    rect.right += padding.right;
                    entity_builder.with(Hitbox::from(rect))
                } else {
                    panic!(
                        "HitboxConfig::SizePadding entity doesn't have a Size"
                    );
                }
            }
        }
    }
}
