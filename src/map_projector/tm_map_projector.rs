use crate::coordinate::{Cartesian, Geographic};
use crate::map_projector_info::TmMapProjectorInfo;

use super::base::MapProjector;

#[derive(Debug)]
pub struct TmMapProjector {
    map_projector_info: TmMapProjectorInfo,
    projector: proj::Proj,
}
impl MapProjector for TmMapProjector {
    fn to_coord(&self, from_coord: &Geographic) -> Result<Cartesian, proj::ProjError> {
        let projected_coord: geo_types::Coord<f64> =
            self.projector.project(from_coord.into(), false)?;
        Ok(Cartesian {
            z: from_coord.ele - self.map_projector_info.map_origin.altitude,
            ..projected_coord.into()
        })
    }
    fn to_lat_lon(&self, from_coord: &Cartesian) -> Result<Geographic, proj::ProjError> {
        let projected_coord: geo_types::Coord<f64> =
            self.projector.project(from_coord.into(), true)?;
        Ok(Geographic {
            ele: from_coord.z + self.map_projector_info.map_origin.altitude,
            ..projected_coord.into()
        })
    }
}
impl From<&TmMapProjectorInfo> for TmMapProjector {
    fn from(info: &TmMapProjectorInfo) -> TmMapProjector {
        let tm_projection = format!(
            "+proj=tmerc +lon_0={lon} +lat_0={lat} +k_0=0.9996 +ellps={ellps}",
            lon = info.map_origin.longitude,
            lat = info.map_origin.latitude,
            ellps = info.vertical_datum
        );
        let latlong_projection = format!("+proj=longlat +datum={}", info.vertical_datum);
        let pipeline_projection = format!(
            "+proj=pipeline +step {} +step {}",
            latlong_projection, tm_projection
        );
        let projector = proj::Proj::new(&pipeline_projection).unwrap();

        TmMapProjector {
            map_projector_info: info.clone(),
            projector,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::map_projector_info::MapOrigin;

    use super::*;

    use serde::Deserialize;
    use std::fs;
    use std::path::PathBuf;

    #[derive(Debug, Deserialize)]
    struct TestCases(Vec<(MapOrigin, Vec<(Geographic, Cartesian)>)>);

    fn get_testcases() -> TestCases {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures/tm_testcase.json");
        let data = fs::read_to_string(path).expect("Failed to read tm_testcase.json");
        serde_json::from_str(&data).expect("Failed to parse JSON")
    }

    #[test]
    fn tm() {
        let testcases = get_testcases();

        for (map_origin, testcases) in testcases.0 {
            let info = TmMapProjectorInfo {
                vertical_datum: String::from("WGS84"),
                map_origin,
            };
            let projector = TmMapProjector::from(&info);

            for (geographic, expected) in testcases {
                assert_eq!(projector.to_coord(&geographic).unwrap(), expected);
                assert_eq!(projector.to_lat_lon(&expected).unwrap(), geographic);
            }
        }
    }
}
