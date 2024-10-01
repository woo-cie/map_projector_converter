use crate::coordinate::{Cartesian, Geographic};
use crate::map_projector_info::TmMapProjectorInfo;

use super::base::MapProjector;

#[derive(Debug)]
pub struct TmMapProjector {}
impl MapProjector for TmMapProjector {
    fn to_coord(&self, _from_coord: &Geographic) -> Cartesian {
        todo!()
    }
    fn to_lat_lon(&self, _from_coord: &Cartesian) -> Geographic {
        todo!()
    }
}
impl From<&TmMapProjectorInfo> for TmMapProjector {
    fn from(_v: &TmMapProjectorInfo) -> TmMapProjector {
        TmMapProjector {}
    }
}
