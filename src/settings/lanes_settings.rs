// resources/settings/lanes.ron

#[derive(Deserialize, Clone)]
pub struct LanesSettings {
    pub count: usize,
    pub width: f32,
    pub paddiug: f32,
}
