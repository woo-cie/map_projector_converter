use crate::coordinate::{Cartesian, Geographic};
use crate::map_projector_info::MgrsMapProjectorInfo;

use super::base::{MapProjector, MapProjectorError, MapProjectorResult};

#[derive(Debug)]
pub struct MgrsMapProjector {
    map_projector_info: MgrsMapProjectorInfo,
    mgrs: geoconvert::Mgrs,
    utm_projector: proj::Proj,
}
impl MgrsMapProjector {
    fn get_mgrs_grid_size(&self) -> f64 {
        let precision = self.mgrs.precision();
        10.0_f64.powf(5.0 - precision as f64)
    }
}
impl MapProjector for MgrsMapProjector {
    fn to_coord(&self, from_coord: &Geographic) -> MapProjectorResult<Cartesian> {
        let utm_coord: geo_types::Coord<f64> =
            self.utm_projector.project(from_coord.into(), false)?;
        Ok(Cartesian {
            x: utm_coord.x - self.mgrs.easting() + self.get_mgrs_grid_size() / 2.0,
            y: utm_coord.y - self.mgrs.northing() + self.get_mgrs_grid_size() / 2.0,
            z: from_coord.ele,
        })
    }
    fn to_lat_lon(&self, from_coord: &Cartesian) -> MapProjectorResult<Geographic> {
        let utm_coord = Cartesian {
            x: from_coord.x + self.mgrs.easting() - self.get_mgrs_grid_size() / 2.0,
            y: from_coord.y + self.mgrs.northing() - self.get_mgrs_grid_size() / 2.0,
            ..from_coord.clone()
        };
        let latlong_coord: geo_types::Coord<f64> =
            self.utm_projector.project(utm_coord.into(), true)?;
        Ok(Geographic {
            ele: from_coord.z,
            ..latlong_coord.into()
        })
    }
}
impl TryFrom<&MgrsMapProjectorInfo> for MgrsMapProjector {
    type Error = MapProjectorError;
    fn try_from(info: &MgrsMapProjectorInfo) -> MapProjectorResult<Self> {
        let mgrs = geoconvert::Mgrs::parse_str(&info.mgrs_grid)?;

        let utm_projection = format!(
            "+proj=utm +zone={zone} +k_0=0.9996 +ellps={ellps}",
            zone = mgrs.zone(),
            ellps = info.vertical_datum
        );
        let latlong_projection = format!("+proj=longlat +datum={}", info.vertical_datum);
        let pipeline_projection = format!(
            "+proj=pipeline +step {} +step {}",
            latlong_projection, utm_projection
        );
        let utm_projector = proj::Proj::new(&pipeline_projection)?;

        Ok(MgrsMapProjector {
            map_projector_info: info.clone(),
            mgrs,
            utm_projector,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Deserialize;
    use std::fs;
    use std::path::PathBuf;

    #[derive(Debug, Deserialize)]
    struct TestCases(Vec<(String, Vec<(Geographic, Cartesian)>)>);

    fn get_testcases() -> TestCases {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures/mgrs_testcase.json");
        let data = fs::read_to_string(path).expect("Failed to read mgrs_testcase.json");
        serde_json::from_str(&data).expect("Failed to parse JSON")
    }

    #[test]
    fn mgrs() {
        let testcases = get_testcases();

        for (mgrs_grid, testcases) in testcases.0 {
            let info = MgrsMapProjectorInfo {
                vertical_datum: String::from("WGS84"),
                mgrs_grid,
            };
            let projector = MgrsMapProjector::try_from(&info).unwrap();

            for (geographic, expected) in testcases {
                assert_eq!(projector.to_coord(&geographic).unwrap(), expected);
                assert_eq!(projector.to_lat_lon(&expected).unwrap(), geographic);
            }
        }
    }
}
