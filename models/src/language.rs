use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Language {
    mapping: HashMap<String, String>,
}

mod test {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use crate::MC_DATA_DIR;

    #[test]
    fn test_language() {
        for version_folder in std::fs::read_dir(MC_DATA_DIR).unwrap() {
            let dir = version_folder.unwrap();
            let mut path = dir.path();
            path.push("language.json");

            if path.exists() {
                println!("{}", path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _language: Language = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}
