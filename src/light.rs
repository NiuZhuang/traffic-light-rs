pub(crate) mod house_light;
pub(crate) mod traffic_light;
use std::fmt::Debug;

pub(crate) trait Light {
    fn get_name(&self) -> &str;
    fn get_state(&self) -> &dyn Debug;
}
