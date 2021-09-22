use std::path::{self, Path, PathBuf};

use generators::{generate, GenerationConfig};

fn main() {
    println!("cargo:rerun-if-changed=src/");

    let config = GenerationConfig {
        biome_data: Path::new("./minecraft-data/data/pc/1.17/biomes.json").to_path_buf(),
        biome_out: Path::new("./src/biome.rs").to_path_buf(),
    };

    generate(config);
}
