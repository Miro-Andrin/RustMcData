use std::path::PathBuf;

mod biome;
mod block;
mod entity;
mod inventory;
mod item;
mod particle;
mod simplified_blocks;
mod tags;

pub struct GenerationConfig {
    pub biome_out: PathBuf
}


pub fn generate(conf: GenerationConfig) {
    biome::generate(conf.biome_out);
    
}
