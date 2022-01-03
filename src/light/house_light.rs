use super::Light;
use std::fmt::{Debug, Display};

pub struct HouseLight {
    on: bool,
}
impl Display for HouseLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "House light is {}", if self.on { "on" } else { "off" })
    }
}
impl HouseLight {
    pub fn new() -> Self {
        Self { on: false }
    }
}

impl Light for HouseLight {
    fn get_name(&self) -> &str {
        "House light"
    }

    fn get_state(&self) -> &dyn Debug {
        &self.on
    }
}
