use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Commands {
    root: RootNode,
    parsers: Vec<ParseInfo>
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Node {
    #[serde(rename = "literal")]
    Literal(LiteralNode),
    #[serde(rename = "argument")]
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
    name: String,
    executable: bool,
    redirects: Vec<String>,
    children: Vec<Node>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArgumentNode {
    name: String,
    executable: bool,
    redirects: Vec<String>,
    children: Vec<Node>,
    parser: Option<ParseInfo>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParseInfo {
    parser: String,
    modifier: Option<Modifier>,
    examples: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Modifier {
    amount: Option<String>,
    r#type: Option<String>,
    min: Option<f32>,
}



#[cfg(test)]
mod test {
    use super::*;
    const MC_DATA_DIR: &str = "./minecraft-data/data/pc/";

    #[test]
    fn test_commands() {
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
