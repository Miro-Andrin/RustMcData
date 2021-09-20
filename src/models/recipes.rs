use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOf<A, B> {
    A(A),
    B(B)
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(transparent)]
pub struct Recipies {
    value : HashMap<String, OneOf<Vec<Recipe>, NewRecipe>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewRecipe {
    name: Option<String>,
    r#type: NewRecipeType,
    ingredients: Vec<serde_json::Value>,
    input: Option<Vec<serde_json::Value>>,
    output: Vec<serde_json::Value>,
    priority: Option<f64> // Only used in bedrock
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
    Campfire
}

type Recipe = OneOf<ShapedRecipe, ShapelessRecipe>;
#[derive(Debug,Serialize,Deserialize)]
pub struct ShapedRecipe {
    result: RecipeItem ,  // todo
    #[serde(rename = "inShape")]
    in_shape: Shape, //todo
    #[serde(rename = "outShape")]
    out_shape: Option<Shape>, // todo
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ShapelessRecipe {
    result: RecipeItem ,  
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
    rows: Vec<Vec<RecipeItem>>
}




mod test {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    const MC_DATA_DIR: &str = "./minecraft-data/data/pc/";

    #[test]
    fn test_language() {
        for version_folder in std::fs::read_dir(MC_DATA_DIR).unwrap() {
            let dir = version_folder.unwrap();
            let mut path = dir.path();
            path.push("recipes.json");

            if path.exists() {
                println!("{}",path.display());
                let contents = std::fs::read_to_string(path).unwrap();
                let _particle: Recipies = serde_json::from_str(&contents).unwrap();
            }
        }
    }
}



// #[derive(Debug, Deserialize, Serialize)]
// #[serde(transparent)]
// pub struct Recipies {
//     values: HashMap<String, Vec<Recipie>>
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(untagged)]
// pub enum Recipie {
//     Shaped(ShapedRecipe),
//     Shapeless(ShapelessRecipe)
// }
// #[derive(Debug, Deserialize, Serialize)]
// pub struct ShapedRecipe {
//     result: serde_json::Value,
//     inShape: serde_json::Value,
//     outShape: Option<serde_json::Value>
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct ShapelessRecipe {
//     result : serde_json::Value ,
//     ingredients : serde_json::Value , 
// }

// ---------------------------------------

// #[derive(Debug, Deserialize, Serialize)]
// pub enum SchemaVersion {
//     Old(Vec<Recipe>),
//     //New(NewRecipeScheme)
// }

// // Definitions
// type Metadata = u64;
// type ID = Option<u64>;
// type Count = u64;
// type IdMetadataArray = Vec<Option<u64>>;

// #[derive(Debug, Deserialize, Serialize)]
// struct IdMetadataCountObject {
//     Id: ID,
//     metadata: Option<Metadata>, 
//     count: Option<Count>, 
// }

// #[derive(Debug, Deserialize, Serialize)]
// enum RecipeItem {
//     Id(ID),
//     Array(IdMetadataArray),
//     Obj(IdMetadataCountObject)
// }

// // #[derive(Debug, Deserialize, Serialize)]
// // pub enum RecipeContainer {
// //     Array(Vec<Recipe>)
// // }


// type ShapeRow = Vec<RecipeItem>;
// type Shape = Vec<ShapeRow>;
// type Ingredients = Vec<RecipeItem>; // At least one


// #[derive(Debug, Deserialize, Serialize)]
// pub struct ShapedRecipe {
//     result: RecipeItem,
//     inShape: Option<Shape>,
//     outShape: Shape,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct ShapelessRecipe {
//     result: RecipeItem,
//     ingredients: Ingredients,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub enum Recipe {
//     Shaped(ShapedRecipe),
//     Shapeless(ShapelessRecipe)
// }



// // New recipe schema 
// #[derive(Debug, Deserialize, Serialize)]
// pub struct NewRecipeScheme {
//     name: Option<String>, // unique id
//     r#type: NewRecipeSchemeType,
//     ingredients: Vec<serde_json::Value>, //Todo: figure out what this is?
//     input: Option<Vec<serde_json::Value>>, //Todo: figure out what this is?
//     output: Vec<serde_json::Value>, //Todo: figure out what this is?}
//     priority: Vec<f64>, // bedrock only
// }
// #[derive(Debug, Deserialize, Serialize)]
// pub enum NewRecipeSchemeType {
//     multi,
//     cartography_table,
//     shapeless,
//     stonecutter,
//     crafting_table,
//     shaped,
//     shulker_box,
//     furnace,
//     blast_furnace,
//     smoker,
//     soul_campfire,
//     campfire
// }

