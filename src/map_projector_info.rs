use std::{fs::File, io, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MgrsMapProjectorInfo {
    vertical_datum: String,
    pub mgrs_grid: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MapOrigin {
    pub longitude: f64,
    pub latitude: f64,
    altitude: f64,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TmMapProjectorInfo {
    vertical_datum: String,
    pub map_origin: MapOrigin,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "projector_type")]
pub enum MapProjectorInfo {
    #[allow(clippy::upper_case_acronyms)]
    MGRS(MgrsMapProjectorInfo),
    TransverseMercator(TmMapProjectorInfo),
}

impl MapProjectorInfo {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<MapProjectorInfo> {
        let f = File::open(path)?;
        let r = BufReader::new(f);
        Ok(serde_yaml::from_reader(r).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::map_projector_info::{MgrsMapProjectorInfo, TmMapProjectorInfo};

    use super::{MapOrigin, MapProjectorInfo};

    #[test]
    fn mgrs() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures/mgrs_map_projector_info.yaml");

        let val = MapProjectorInfo::from_path(path).unwrap();
        assert_eq!(
            val,
            MapProjectorInfo::MGRS(MgrsMapProjectorInfo {
                vertical_datum: String::from("WGS84"),
                mgrs_grid: String::from("54STH")
            })
        );
    }

    #[test]
    fn tm() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures/tm_map_projector_info.yaml");

        let val = MapProjectorInfo::from_path(path).unwrap();
        assert_eq!(
            val,
            MapProjectorInfo::TransverseMercator(TmMapProjectorInfo {
                vertical_datum: String::from("WGS84"),
                map_origin: MapOrigin {
                    longitude: 137.5882546000352,
                    latitude: 37.89810637710265,
                    altitude: 0.0
                }
            })
        );
    }
}
