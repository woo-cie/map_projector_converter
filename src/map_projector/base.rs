use crate::coordinate::{Cartesian, Geographic};
use crate::map_projector_info::MapProjectorInfo;

use super::mgrs_map_projector::MgrsMapProjector;
use super::tm_map_projector::TmMapProjector;

#[derive(Debug, thiserror::Error)]
pub enum MapProjectorError {
    #[error("Projの作成エラー: {0}")]
    CreateProjector(#[from] proj::ProjCreateError),
    #[error("Projのエラー: {0}")]
    Projection(#[from] proj::ProjError),
    #[error("GeoConvertのエラー: {0}")]
    Mgrs(#[from] geoconvert::Error),
}
pub type MapProjectorResult<T> = anyhow::Result<T, MapProjectorError>;
pub trait MapProjector: std::fmt::Debug {
    fn to_lat_lon(&self, from_coord: &Cartesian) -> MapProjectorResult<Geographic>;
    fn to_coord(&self, from_coord: &Geographic) -> MapProjectorResult<Cartesian>;
}

impl TryFrom<&MapProjectorInfo> for Box<dyn MapProjector> {
    type Error = MapProjectorError;
    fn try_from(value: &MapProjectorInfo) -> anyhow::Result<Self, Self::Error> {
        match value {
            MapProjectorInfo::MGRS(v) => Ok(Box::<MgrsMapProjector>::new(v.try_into()?)),
            MapProjectorInfo::TransverseMercator(v) => {
                Ok(Box::<TmMapProjector>::new(v.try_into()?))
            }
        }
    }
}
