// resources/settings/player.ron

use super::entity_components::EntityComponents;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PlayerSettings {
    pub components: EntityComponents,
}
