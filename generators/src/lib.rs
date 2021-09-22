use std::path::{Path, PathBuf};

mod biome;
mod block;
mod entity;
mod inventory;
mod item;
mod particle;
mod simplified_blocks;
mod tags;

pub struct GenerationConfig {
    pub biome_out: PathBuf,
    pub biome_data: PathBuf,
}

/// Returns the string as UpperCamelCase
pub(crate) fn camel_case(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut new_word: bool = true;

    for c in name.chars() {
        if c == '_' {
            new_word = true;
        } else if new_word {
            result.push(c.to_ascii_uppercase());
            new_word = false;
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }

    result
}

pub fn generate(conf: GenerationConfig) {
    let biome_data = models::Biomes::from_file(conf.biome_data).unwrap();

    biome::generate(biome_data, conf.biome_out);
}
