use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Biomes {
    pub biomes: Vec<Biome>,
}

impl Biomes {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        let contents = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&contents)?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Biome {
    /// The unique identifies for a biome
    pub id: u64,
    /// The name of a biome
    pub name: String,
    /// The category of a biome
    category: String,
    /// The temperature of a biome between -1 and 2
    pub temperature: f64,
    /// The type of precipitation: none, rain or snow
    precipitation: Precipitation,
    /// The depth of a biome
    depth: f64,
    /// The dimension of a biome: overworld, nether or end
    dimension: Dimension,
    /// The display name of a biome
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// The color in a biome
    color: u64,
    /// How much rain there is in a biome, between 0 and 1.
    pub rainfall: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Precipitation {
    None,
    Rain,
    Snow,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Dimension {
    Overworld,
    Nether,
    End,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::MC_DATA_DIR;

    #[test]
    fn test_biome_model() {
        for version_folder in std::fs::read_dir(MC_DATA_DIR).unwrap() {
            let dir = version_folder.unwrap();
            let mut biome_path = dir.path();
            biome_path.push("biomes.json");

            if biome_path.exists() {
                println!("{}", biome_path.display());
                let contents = std::fs::read_to_string(biome_path).unwrap();
                let _biomes: Biomes = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}
