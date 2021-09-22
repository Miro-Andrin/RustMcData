use std::{collections::HashMap, fs::File, path::Path};

use quote::format_ident;
use quote::quote;
use std::io::Write;

use crate::camel_case;

/*

let blocks = vec!["stone", "gravel"];

let block_idents = blocks
    .iter()
    .map(|block| format_ident!("{}", block))
    .collect::<Vec<_>>();

let block_transparency = vec![true, true];
let block_emit_light = vec![2, 2];

quote! {
    pub enum Block {
        #(#block_idents),*
    }

    impl Block {
        pub fn is_transparent(&self) -> bool {
            match &self {
                #(Self::#block_idents => #block_transparency),*
            }
        }

        pub fn emit_light(&self) -> u8 {
            match &self {
                #(Self::#block_idents => #block_emit_light),*
            }
        }
    }
}
*/

/**
   'biome_json' is where the biome json file is located.
   'out' is where the generated biome file should be stored.
*/
pub fn generate(inn: models::Biomes, out: impl AsRef<Path>) {
    let mut variants = Vec::<String>::new();
    let mut ids = Vec::new();
    let mut names = Vec::new();
    let mut display_names = Vec::new();
    let mut rainfalls = Vec::new();
    let mut temperatures = Vec::new();

    for biome in inn.biomes {
        let variant = camel_case(&biome.name);
        variants.push(variant.clone());

        ids.push(biome.id);
        names.push(biome.name.clone());
        display_names.push(biome.display_name.clone());
        rainfalls.push(biome.rainfall as f32);
        temperatures.push(biome.temperature.clone() as f32);
    }

    let biome_idents = variants
        .iter()
        .map(|variant| format_ident!("{}", variant))
        .collect::<Vec<_>>();

    let data = quote! {
        pub enum BiomeKind {
            #(#biome_idents),*
        }

        pub fn id(&self) -> u64 {
            match &self {
                #(Self::#biome_idents => #ids),*
            }
        }

        pub fn name(&self) -> &'static str {
            match &self {
                #(Self::#biome_idents => #names),*
            }
        }

        pub fn display_name(&self) -> &'static str {
            match &self {
                #(Self::#biome_idents => #display_names),*
            }
        }

        pub fn rainfall(&self) -> f32 {
            match &self {
                #(Self::#biome_idents => #rainfalls),*
            }
        }

        pub fn temperature(&self) -> f32 {
            match &self {
                #(Self::#biome_idents => #temperatures),*
            }
        }

    };

    let mut w = File::create(out).unwrap();
    write!(&mut w, "{}", data).unwrap();
}
