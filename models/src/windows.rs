use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Windows {
    windows: Vec<Window>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Window {
    /// The unique identifier for the window.
    id: String,
    /// "The default displayed name of the window.
    name: String,
    /// The slots displayed in the window.
    slots: Option<Vec<WindowSlot>>,
    /// Names of the properties of the window.
    properties: Option<Vec<String>>,
    opened_with: Option<OpenedWith>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WindowSlot {
    /// The name of the slot or slot range.
    name: String,
    /// The position of the slot or beginning of the slot range.
    index: u64,
    /// The size of the slot range.
    size: Option<u64>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenedWith {
    r#type: String,
    id: u64,
}

mod test {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use crate::MC_DATA_DIR;

    #[test]
    fn test_windows() {
        for version_folder in std::fs::read_dir(MC_DATA_DIR).unwrap() {
            let dir = version_folder.unwrap();
            let mut path = dir.path();
            path.push("windows.json");

            if path.exists() {
                println!("{}",path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _windows: Windows = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}

