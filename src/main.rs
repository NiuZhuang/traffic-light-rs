use std::fmt::Display;

#[derive(Debug)]
#[allow(dead_code)]
enum TrafficLightColor {
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
struct TrafficLight {
    color: TrafficLightColor,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            color: TrafficLightColor::Red,
        }
    }
    #[allow(dead_code)]
    pub fn get_state(&self) -> &TrafficLightColor {
        &self.color
    }

    pub fn turn_green(&mut self) {
        self.color = TrafficLightColor::Green
    }
}

impl Display for TrafficLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Traffict light is {}", self.color)
    }
}

fn main() {
    let mut light = TrafficLight::new();
    println!("{:?}", light);
    light.turn_green();
    println!("{:?}", light);
}
