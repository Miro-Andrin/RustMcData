use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    #[serde(rename = "minecraftVersion")]
    minecraft_version: String,
    version: u64,
    #[serde(rename = "majorVersion")]
    major_version: String
}

mod test {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    const MC_DATA_DIR: &str = "./minecraft-data/data/pc/";

    #[test]
    fn test_version() {
        for version_folder in std::fs::read_dir(MC_DATA_DIR).unwrap() {
            let dir = version_folder.unwrap();
            let mut path = dir.path();
            path.push("version.json");

            if path.exists() {
                println!("{}",path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _version: Version = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}
