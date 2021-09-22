use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProtocolVersions {
    entries: Vec<ProtocolVersion>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProtocolVersion {
    version: i64,
    #[serde(rename = "dataVersion")]
    data_version: Option<i64>,
    #[serde(rename = "minecraftVersion")]
    minecraft_version: String,
    #[serde(rename = "majorVersion")]
    major_version: String,
    #[serde(rename = "useNetty")]
    uses_netty: Option<bool>,
    #[serde(rename = "releaseType")]
    release_type: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;
    const PATH: &str = "./minecraft-data/data/pc/common/protocolVersions.json";

    #[test]
    fn test_block_loot() {
        let contents = std::fs::read_to_string(PATH).unwrap();
        let _shapes: ProtocolVersions = serde_json::from_str(&contents).unwrap();
    }
}
