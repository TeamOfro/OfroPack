use serde::{Deserialize, Serialize};

use crate::constants::ItemModelParent;

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemModel {
    pub parent: ItemModelParent,
    pub textures: Textures,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Textures {
    pub layer0: String,
}

impl ItemModel {
    pub fn new(parent: ItemModelParent, custom_model_data: &str) -> Self {
        Self {
            parent,
            textures: Textures {
                layer0: format!("item/{}", custom_model_data),
            },
        }
    }
}
