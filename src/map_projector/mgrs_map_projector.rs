use crate::coordinate::{Cartesian, Geographic};
use crate::map_projector_info::MgrsMapProjectorInfo;

use super::base::MapProjector;

#[derive(Debug)]
pub struct MgrsMapProjector {}
impl MapProjector for MgrsMapProjector {
    fn to_coord(&self, _from_coord: &Geographic) -> Cartesian {
        todo!()
    }
    fn to_lat_lon(&self, _from_coord: &Cartesian) -> Geographic {
        todo!()
    }
}
impl From<&MgrsMapProjectorInfo> for MgrsMapProjector {
    fn from(_v: &MgrsMapProjectorInfo) -> MgrsMapProjector {
        MgrsMapProjector {}
    }
}
