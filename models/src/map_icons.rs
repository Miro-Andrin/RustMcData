use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MapIcons {
    icons: Vec<MapIcon>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MapIcon {
    id: u64,
    name: String,
    /// Description of the map icon's appearance
    appearance: String,
    #[serde(rename = "visibleInItemFrame")]
    visible_in_item_frame: bool,
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
            path.push("mapIcons.json");

            if path.exists() {
                println!("{}", path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _shapes: MapIcons = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}
