use super::Light;
use std::fmt::{Debug, Display};

#[derive(Debug)]
#[allow(dead_code)]
pub enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl Display for TrafficLightColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_string = match self {
            TrafficLightColor::Green => "green",
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
        };
        write!(f, "{}", color_string)
    }
}

#[derive(Debug)]
pub struct TrafficLight {
    color: TrafficLightColor,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            color: TrafficLightColor::Red,
        }
    }
    #[allow(dead_code)]
    pub fn turn_green(&mut self) {
        self.color = TrafficLightColor::Green
    }
}

impl Display for TrafficLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Traffict light is {}", self.color)
    }
}

impl Light for TrafficLight {
    fn get_name(&self) -> &str {
        "Traffic light"
    }

    fn get_state(&self) -> &dyn Debug {
        &self.color
    }
}
