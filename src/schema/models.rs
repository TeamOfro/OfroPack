use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemModel {
    pub parent: String,
    pub textures: Textures,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Textures {
    pub layer0: String,
}
