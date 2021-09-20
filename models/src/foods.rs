use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Foods {
    foods: Vec<Food>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Food {
    id: u64,
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "stackSize")]
    stack_size: u64,
    name: String,
    #[serde(rename = "foodPoints")]
    food_points: f64, // zero or more
    saturation: f64,  // zero or more
    #[serde(rename = "effectiveQuality")]
    effective_quality: f64,
    #[serde(rename = "saturationRatio")]
    saturation_ratio: f64,
    variations: Option<Vec<Variation>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Variation {
    metadata: u64,
    #[serde(rename = "displayName")]
    display_name: String,
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
            path.push("foods.json");

            if path.exists() {
                println!("{}",path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _shapes: Foods = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}