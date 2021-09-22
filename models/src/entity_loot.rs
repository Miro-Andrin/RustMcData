use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EntityLoots {
    entries: Vec<EntityLoot>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityLoot {
    entity: String,
    drops: Vec<ItemDrop>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemDrop {
    item: String,
    #[serde(rename = "dropChance")]
    drop_chance: f64,
    #[serde(rename = "stackSizeRange")]
    stack_size_range: [u64; 2],
    #[serde(rename = "playerKill")]
    player_kill: Option<bool>,
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
            path.push("entityLoot.json");

            if path.exists() {
                println!("{}", path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _shapes: EntityLoots = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}
