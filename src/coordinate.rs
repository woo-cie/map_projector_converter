use serde::{Deserialize, Serialize};

const EPSILON_DEGREE: f64 = 1e-11;
const EPSILON_METER: f64 = 1e-4;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Geographic {
    pub lat: f64,
    pub lon: f64,
    pub ele: f64,
}
impl Geographic {
    fn to_geo_coord(&self) -> geo_types::Coord<f64> {
        geo_types::Coord {
            x: self.lon.to_radians(),
            y: self.lat.to_radians(),
        }
    }
}
impl From<geo_types::Coord<f64>> for Geographic {
    fn from(v: geo_types::Coord<f64>) -> Self {
        Geographic {
            lon: v.x.to_degrees(),
            lat: v.y.to_degrees(),
            ele: 0.0, // 標高が不明な場合は 0.0 にする
        }
    }
}
impl From<Geographic> for geo_types::Coord<f64> {
    fn from(v: Geographic) -> Self {
        v.to_geo_coord()
    }
}
impl From<&Geographic> for geo_types::Coord<f64> {
    fn from(v: &Geographic) -> Self {
        v.to_geo_coord()
    }
}
impl PartialEq for Geographic {
    fn eq(&self, other: &Self) -> bool {
        (self.lat - other.lat).abs() < EPSILON_DEGREE
            && (self.lon - other.lon).abs() < EPSILON_DEGREE
            && (self.ele - other.ele).abs() < EPSILON_METER
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cartesian {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Cartesian {
    fn to_geo_coord(&self) -> geo_types::Coord<f64> {
        geo_types::Coord {
            x: self.x,
            y: self.y,
        }
    }
}
impl From<geo_types::Coord<f64>> for Cartesian {
    fn from(v: geo_types::Coord<f64>) -> Self {
        Cartesian {
            x: v.x,
            y: v.y,
            z: 0.0,
        }
    }
}
impl From<Cartesian> for geo_types::Coord<f64> {
    fn from(v: Cartesian) -> Self {
        v.to_geo_coord()
    }
}
impl From<&Cartesian> for geo_types::Coord<f64> {
    fn from(v: &Cartesian) -> Self {
        v.to_geo_coord()
    }
}
impl PartialEq for Cartesian {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON_METER
            && (self.y - other.y).abs() < EPSILON_METER
            && (self.z - other.z).abs() < EPSILON_METER
    }
}
