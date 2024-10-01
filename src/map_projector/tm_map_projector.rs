use crate::coordinate::{Cartesian, Geographic};
use crate::map_projector_info::{MapProjectorInfo, TmMapProjectorInfo};

use super::map_projector::MapProjector;

#[derive(Debug)]
pub struct TmMapProjector {}
impl MapProjector for TmMapProjector {
    fn to_coord(&self, from_coord: &Geographic) -> Cartesian {
        todo!()
    }
    fn to_lat_lon(&self, from_coord: &Cartesian) -> Geographic {
        todo!()
    }
}
impl From<&TmMapProjectorInfo> for TmMapProjector {
    fn from(v: &TmMapProjectorInfo) -> TmMapProjector {
        TmMapProjector {}
    }
}
