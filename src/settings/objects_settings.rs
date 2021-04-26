// resources/settings/objects.ron

use super::entity_components::EntityComponents;
use crate::level_loader::ObjectType;
use deathframe::components::prelude::Merge;
use std::collections::HashMap;

type ObjectsSettingsMap = HashMap<ObjectType, ObjectSettings>;

#[derive(Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, from = "ObjectsSettingsMap")]
pub struct ObjectsSettings {
    pub objects: ObjectsSettingsMap,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ObjectSettings {
    pub components:  EntityComponents,
    #[serde(default)]
    pub spritesheet: Option<String>,
}

impl From<ObjectsSettingsMap> for ObjectsSettings {
    fn from(objects: ObjectsSettingsMap) -> Self {
        Self { objects }
    }
}

impl Merge for ObjectsSettings {
    fn merge(&mut self, other: Self) {
        self.objects.extend(&mut other.objects.into_iter())
    }
}
