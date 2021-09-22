use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOf<A, B> {
    A(A),
    B(B),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Recipies {
    value: HashMap<String, OneOf<Vec<Recipe>, NewRecipe>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewRecipe {
    name: Option<String>,
    r#type: NewRecipeType,
    ingredients: Vec<u64>,
    priority: Option<f64>, // Only used in bedrock
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NewRecipeType {
    Multi,
    CartographyTable,
    Shapeless,
    Stonecutter,
    CraftingTable,
    Shaped,
    ShulkerBox,
    Furnace,
    BlastFurnace,
    Smoker,
    SoulCampfire,
    Campfire,
}

type Recipe = OneOf<ShapedRecipe, ShapelessRecipe>;
#[derive(Debug, Serialize, Deserialize)]
pub struct ShapedRecipe {
    result: RecipeItem, // todo
    #[serde(rename = "inShape")]
    in_shape: Shape, //todo
    #[serde(rename = "outShape")]
    out_shape: Option<Shape>, // todo
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapelessRecipe {
    result: RecipeItem,
    ingredients: Vec<RecipeItem>, //todo
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecipeItem {
    Id(Option<u64>),
    IdMetadataArray(Vec<Option<u64>>), // todo might be wrong
    IdMetadataCountObj {
        id: u64,
        metadata: Option<u64>,
        count: Option<u64>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Shape {
    // This is the only case were RecipeItem::Id or metadata can be null.
    rows: Vec<Vec<RecipeItem>>,
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
            path.push("recipes.json");

            if path.exists() {
                println!("{}", path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _particle: Recipies = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}
