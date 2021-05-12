pub enum ZoneProgressionMode {
    Progression,
    Infinite,
}

impl ZoneProgressionMode {
    pub fn from_is_infinite(is_infinite: bool) -> Self {
        if is_infinite {
            Self::Infinite
        } else {
            Self::default()
        }
    }
}

impl Default for ZoneProgressionMode {
    fn default() -> Self {
        Self::Progression
    }
}
