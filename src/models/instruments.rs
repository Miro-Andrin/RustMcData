use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Instruments {
    instruments: Vec<Instrument>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Instrument {
    /// The unique identifier for an instrument.
    id: u64,
    /// The name of an instrument.
    name: String,
    /// The sound ID played by this instrument.
    sound: Option<String>,
}

mod test {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    const MC_DATA_DIR: &str = "./minecraft-data/data/pc/";

    #[test]
    fn test_instruments() {
        for version_folder in std::fs::read_dir(MC_DATA_DIR).unwrap() {
            let dir = version_folder.unwrap();
            let mut path = dir.path();
            path.push("instruments.json");

            if path.exists() {
                println!("{}",path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _instruments: Instruments = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}
