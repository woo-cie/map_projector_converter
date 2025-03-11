use std::path::Path;

use crate::map_projector::base::MapProjectorResult;

use super::coordinate::Cartesian;
use super::map_projector::base::MapProjector;
use super::map_projector_info::MapProjectorInfo;

#[derive(Debug)]
pub struct MapProjectorConverter {
    from_projector: Box<dyn MapProjector>,
    to_projector: Box<dyn MapProjector>,
}
impl MapProjectorConverter {
    pub fn from_path<P: AsRef<Path>>(
        input_yaml_path: P,
        output_yaml_path: P,
    ) -> anyhow::Result<MapProjectorConverter> {
        let input_map_projector_info = MapProjectorInfo::from_path(input_yaml_path)?;
        let input_map_projector: Box<dyn MapProjector> = (&input_map_projector_info).try_into()?;

        let output_map_projector_info = MapProjectorInfo::from_path(output_yaml_path)?;
        let output_map_projector: Box<dyn MapProjector> =
            (&output_map_projector_info).try_into()?;

        Ok(MapProjectorConverter {
            from_projector: input_map_projector,
            to_projector: output_map_projector,
        })
    }

    pub fn convert(&self, coord: Cartesian) -> MapProjectorResult<Cartesian> {
        let lat_lon = self.from_projector.to_lat_lon(&coord)?;
        self.to_projector.to_coord(&lat_lon)
    }
}
