use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Effects {
    effects: Vec<Effect>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Effect {
    /// The unique identifier for an effect.
    id: u64,
    /// The display name of an effect.
    #[serde(rename = "displayName")]
    display_name: String,
    /// The name of an effect.
    name: String,
    /// Whether an effect is positive or negative.
    r#type: EffectType,

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EffectType {
    Good,
    Bad
}

mod test {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    const MC_DATA_DIR: &str = "./minecraft-data/data/pc/";

    #[test]
    fn test_effects() {
        for version_folder in std::fs::read_dir(MC_DATA_DIR).unwrap() {
            let dir = version_folder.unwrap();
            let mut path = dir.path();
            path.push("effects.json");

            if path.exists() {
                println!("{}",path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _effects: Effects = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}
