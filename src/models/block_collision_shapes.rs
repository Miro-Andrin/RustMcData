use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockCollisionShapes {
    blocks: HashMap<String, ShapeIds>,
    shapes: HashMap<String, Vec<[f64; 6]>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ShapeIds {
    Singe(usize),
    Many(Vec<usize>),
}

#[cfg(test)]
mod test {
    use super::*;
    const MC_DATA_DIR: &str = "./minecraft-data/data/pc/";

    #[test]
    fn test_block_colission_model() {
        for version_folder in std::fs::read_dir(MC_DATA_DIR).unwrap() {
            let dir = version_folder.unwrap();
            let mut path = dir.path();
            path.push("blockCollisionShapes.json");

            if path.exists() {
                let contents = std::fs::read_to_string(path).unwrap();
                let _shapes: BlockCollisionShapes = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}
