use serde::{Deserialize, Serialize};
use std::{collections::HashMap};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Blocks {
    blocks: Vec<Block>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
    id: u64,
    #[serde(rename = "displayName")]
    display_name: String,
    name: String,
    hardness: Option<f64>,
    #[serde(rename = "stackSize")]
    stack_size: u64,
    diggable: bool,
    #[serde(rename = "boundingBox")]
    bounding_box: BoundingBox,
    material: Option<String>,
    #[serde(rename = "harvestTools")]
    harvest_tools : Option<HashMap<usize, bool>>,
    variations: Option<Vec<Variation>>,
    states: Option<Vec<State>>,
    drops: Drops,
    transparent: bool,
    #[serde(rename = "emitLight")]
    emit_light: u8, // 0 to 15
    #[serde(rename = "filterLight")]
    filter_light: u8, // 0 to 15
    #[serde(rename = "minStateId")]
    min_state_id: Option<u64>,
    #[serde(rename = "maxStateId")]
    max_state_id: Option<u64>,
    #[serde(rename = "defaultState")]
    default_state: Option<u64>,
    resistance: Option<f64>, // -1 upwrards.
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BoundingBox {
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "empty")]
    Empty
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Drops {
    New(Vec<u64>),
    /// Used in older versions of minecraft.
    Old(Vec<Drop>)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Drop {
    drop: DropInfo
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropInfo {
    Single(u64),
    Object {
        id: u64,
        metadata: u64,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    name: String,
    r#type: String,
    num_values: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variation {
    #[serde(rename = "displayName")]
    display_name: String,
    metadata: u64,
    description: Option<String>
}

#[cfg(test)]
mod test {
    use super::*;
    const MC_DATA_DIR: &str = "./minecraft-data/data/pc/";

    #[test]
    fn test_block() {
        for version_folder in std::fs::read_dir(MC_DATA_DIR).unwrap() {
            let dir = version_folder.unwrap();
            let mut path = dir.path();
            path.push("blocks.json");

            if path.exists() {
                println!("{}",path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _blocks: Blocks = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}
