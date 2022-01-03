use std::fmt::{Display, Debug};

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
    pub fn turn_green(&mut self) {
        self.color = TrafficLightColor::Green
    }
}

impl Display for TrafficLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Traffict light is {}", self.color)
    }
}

struct HouseLight {
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

trait Light {
     fn get_name(&self) -> &str;
     fn get_state(&self) -> &dyn Debug;
}

impl Light for HouseLight {
    fn get_name(&self) -> &str {
        "House light"
    }

    fn get_state(&self) -> &dyn Debug {
        &self.on
    }
}

impl Light for TrafficLight{
    fn get_name(&self) -> &str {
        "Traffic light"
    }

    fn get_state(&self) -> &dyn Debug {
        &self.color
    }
}

fn print_state(light: &impl Light) {
    println!("{}'s state is : {:?}", light.get_name(), light.get_state());
}


fn main() {
    // let mut light = TrafficLight::new();
    // println!("{}", light);
    // println!("{:?}", light);
    // light.turn_green();
    // println!("{:?}", light);

    let traffic_light = TrafficLight::new();
    let house_light = HouseLight::new();

    print_state(&traffic_light);
    print_state(&house_light);
}
