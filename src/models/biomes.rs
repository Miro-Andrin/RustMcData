use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Biomes {
    biomes: Vec<Biome>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Biome {
    /// The unique identifies for a biome
    id: u64,
    /// The name of a biome
    name: String,
    /// The category of a biome
    category: String,
    /// The temperature of a biome between -1 and 2
    temperature: f64,
    /// The type of precipitation: none, rain or snow
    precipitation: Precipitation,
    /// The depth of a biome
    depth: f64,
    /// The dimension of a biome: overworld, nether or end
    dimension: Dimension,
    /// The display name of a biome
    #[serde(rename = "displayName")]
    display_name: String,
    /// The color in a biome
    color: u64,
    /// How much rain there is in a biome, between 0 and 1.
    rainfall: f64,
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
    const MC_DATA_DIR: &str = "./minecraft-data/data/pc/";

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
