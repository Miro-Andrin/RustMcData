use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Materials {
    entries: HashMap<String, HashMap<String, f64>>
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::MC_DATA_DIR;

    #[test]
    fn test_block_loot() {
        for version_folder in std::fs::read_dir(MC_DATA_DIR).unwrap() {
            let dir = version_folder.unwrap();
            let mut path = dir.path();
            path.push("materials.json");
            
            if path.exists() {
                println!("{}",path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _shapes: Materials = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}