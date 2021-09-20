use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tints {
    grass: TintData,
    foliage: TintData,
    water: TintData,
    redstone: RedstoneData,
    constant: TintData
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TintData {
    data: Vec<Data>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    keys: Vec<String>,
    color: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedstoneData {
    data: Vec<RedstoneProps>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedstoneProps {
    keys: Vec<u64>,
    color: i64,
}

mod test {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use crate::MC_DATA_DIR;

    #[test]
    fn test_tints() {
        for version_folder in std::fs::read_dir(MC_DATA_DIR).unwrap() {
            let dir = version_folder.unwrap();
            let mut path = dir.path();
            path.push("tints.json");

            if path.exists() {
                println!("{}",path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _tints: Tints = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}
