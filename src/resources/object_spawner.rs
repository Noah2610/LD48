use crate::level_loader::ObjectType;

#[derive(Default)]
pub struct ObjectSpawner {
    to_spawn: Vec<ObjectToSpawn>,
}

#[derive(Deserialize)]
pub struct ObjectToSpawn {
    #[serde(rename = "type")]
    pub object_type: ObjectType,
    pub pos:         (f32, f32, f32),
    #[serde(default)]
    pub size:        Option<(f32, f32)>,
}

impl ObjectSpawner {
    pub fn spawn(&mut self, object: ObjectToSpawn) {
        self.to_spawn.push(object);
    }

    pub fn objects_to_spawn(&mut self) -> Vec<ObjectToSpawn> {
        self.to_spawn.split_off(0)
    }
}
