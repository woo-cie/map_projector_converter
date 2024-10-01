use crate::coordinate::{Cartesian, Geographic};
use crate::map_projector_info::{MapProjectorInfo, MgrsMapProjectorInfo};

use super::map_projector::MapProjector;

#[derive(Debug)]
pub struct MgrsMapProjector {}
impl MapProjector for MgrsMapProjector {
    fn to_coord(&self, from_coord: &Geographic) -> Cartesian {
        println!("mgrs {:?}", from_coord);
        Cartesian {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    fn to_lat_lon(&self, from_coord: &Cartesian) -> Geographic {
        println!("mgrs {:?}", from_coord);
        Geographic {
            lat: 0.0,
            lon: 0.0,
            ele: 0.0,
        }
    }
}
impl From<&MgrsMapProjectorInfo> for MgrsMapProjector {
    fn from(v: &MgrsMapProjectorInfo) -> MgrsMapProjector {
        MgrsMapProjector {}
    }
}
