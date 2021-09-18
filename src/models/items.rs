use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Items {
    items: Vec<Item>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    id : u64,
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "stackSize")]
    stack_size: u64,
    #[serde(rename = "enchantCategories")]
    enchant_categories: Option<Vec<String>>,
    #[serde(rename = "repairWith")]
    repair_with: Option<Vec<String>>,
    #[serde(rename = "maxDurability")]
    max_durability: Option<u64>,
    name: String,
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
    const MC_DATA_DIR: &str = "./minecraft-data/data/pc/";

    #[test]
    fn test_block_loot() {
        for version_folder in std::fs::read_dir(MC_DATA_DIR).unwrap() {
            let dir = version_folder.unwrap();
            let mut path = dir.path();
            path.push("foods.json");

            if path.exists() {
                println!("{}",path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _shapes: Items = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}