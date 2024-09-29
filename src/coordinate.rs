use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Geographic {
    pub lat: f64,
    pub lon: f64,
    pub ele: f64,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Cartesian {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
