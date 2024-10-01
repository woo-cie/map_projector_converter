use crate::coordinate::{Cartesian, Geographic};
use crate::map_projector_info::MapProjectorInfo;

use super::mgrs_map_projector::MgrsMapProjector;
use super::tm_map_projector::TmMapProjector;
pub trait MapProjector: std::fmt::Debug {
    fn to_lat_lon(&self, from_coord: &Cartesian) -> Geographic;
    fn to_coord(&self, from_coord: &Geographic) -> Cartesian;
}

impl From<&MapProjectorInfo> for Box<dyn MapProjector> {
    fn from(value: &MapProjectorInfo) -> Self {
        match value {
            MapProjectorInfo::MGRS(v) => Box::<MgrsMapProjector>::new(v.into()),
            MapProjectorInfo::TransverseMercator(v) => Box::<TmMapProjector>::new(v.into()),
        }
    }
}
