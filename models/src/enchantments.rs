use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Enchantments {
    enchantments: Vec<Enchantment>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Enchantment {
    id: u64,
    /// The name of an enchantment.
    name: String,
    /// The display name of an enchantment.
    #[serde(rename = "displayName")]
    display_name: String,
    /// The maximum level of an enchantment.
    #[serde(rename = "maxLevel")]
    max_level: u8,
    /// Min cost equation's coefficients a * level + b.
    #[serde(rename = "minCost")]
    min_cost: Cost,
    /// Max cost equation's coefficients a * level + b.
    #[serde(rename = "maxCost")]
    max_cost: Cost,
    /// Can only be found in a treasure, not created.
    #[serde(rename = "treasureOnly")]
    treasure_only: bool,
    /// Is a curse, not an enchantment.
    curse: bool,
    /// List of enchantment not compatibles.
    exclude: Vec<String>,
    /// The category of enchantable items.
    category: String,
    /// Weight of the rarity of the enchantment.
    weight: u8,
    /// Can this enchantment be traded.
    tradeable: bool,
    /// Can this enchantment be discovered.
    discoverable: bool,
    
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cost {
    a: i8,
    b: i8,
}

mod test {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use crate::MC_DATA_DIR;

    #[test]
    fn test_enchantments() {
        for version_folder in std::fs::read_dir(MC_DATA_DIR).unwrap() {
            let dir = version_folder.unwrap();
            let mut path = dir.path();
            path.push("enchantments.json");

            if path.exists() {
                println!("{}",path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _enchantments: Enchantments = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}

