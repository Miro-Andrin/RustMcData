use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Entities {
    values: Vec<Entity>    
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entity {
    id: u64,
    #[serde(rename = "internalId")]
    internal_id: Option<u64>,
    name: String,
    #[serde(rename = "displayName")]
    display_name: String,
    width: Option<f64>,
    height: Option<f64>,
    r#type: String
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
            path.push("entities.json");

            if path.exists() {
                println!("{}",path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _shapes: Entities = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}
