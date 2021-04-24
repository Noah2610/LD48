// resources/settings/lanes.ron

#[derive(Deserialize, Clone)]
pub struct LanesSettings {
    pub count: usize,
    pub spacing: f32,
}
