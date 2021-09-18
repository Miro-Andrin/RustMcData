use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Commands {
    root: RootNode,
    parsers: Vec<ParseInfor>
}
#[derive(Debug, Deserialize, Serialize)]
pub enum Node {
    Literal(LiteralNode),
    Argument(ArgumentNode),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RootNode {
    r#type: String,
    name: String,
    redirects: Vec<String>,
    children: Vec<Node>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LiteralNode {
    r#type: String,
    name: String,
    executable: bool,
    redirects: Vec<String>,
    children: Vec<Node>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArgumentNode {
    r#type: String,
    name: String,
    executable: bool,
    redirects: Vec<String>,
    children: Vec<Node>,
    parser: Option<HashMap<String, String>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParseInfor {
    parser: String,
    modifier: Option<HashMap<String, serde_json::Value>>,
    examples: Vec<String>
}



#[cfg(test)]
mod test {
    use super::*;
    const MC_DATA_DIR: &str = "./minecraft-data/data/pc/";

    #[test]
    fn test_block_loot() {
        for version_folder in std::fs::read_dir(MC_DATA_DIR).unwrap() {
            let dir = version_folder.unwrap();
            let mut path = dir.path();
            path.push("commands.json");

            if path.exists() {
                println!("{}",path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _shapes: Commands = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}
