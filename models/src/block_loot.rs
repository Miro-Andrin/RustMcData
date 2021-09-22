use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BlockLoot {
    items: Vec<BlockLootEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockLootEntry {
    block: String,
    drops: Vec<DropItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DropItem {
    item: String,
    #[serde(rename = "dropChance")]
    drop_chance: f64,
    #[serde(rename = "stackSizeRange")]
    stack_size_range: [Option<i64>; 2],
    #[serde(rename = "blockAge")]
    block_age: Option<u64>,
    #[serde(rename = "silkTouch")]
    silk_touch: Option<bool>,
    #[serde(rename = "noSilkTouch")]
    no_silk_touch: Option<bool>,
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
            path.push("blockLoot.json");

            if path.exists() {
                println!("{}", path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _shapes: BlockLoot = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}
