use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::types::ItemModelParent;

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<ItemModelParent>,
    pub textures: Textures,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Textures(BTreeMap<String, String>);

impl ItemModel {
    pub fn new(parent: ItemModelParent, custom_model_data: &str) -> Self {
        let mut textures = Textures::default();
        textures.add_custom(custom_model_data);
        Self {
            parent: Some(parent),
            textures,
        }
    }
}

impl Textures {
    pub fn add_custom(&mut self, custom_model_data: &str) {
        self.0
            .insert("layer0".to_string(), format!("item/{custom_model_data}"));
    }

    fn add_layer(&mut self, custom_model_data: &str, layer_number: usize) {
        let key = layer_number.to_string();
        self.0
            .insert(key, format!("item/{custom_model_data}/{layer_number}"));
    }

    /// return the number of overwritten layers
    pub fn overwrite(&mut self, custom_model_data: &str) -> usize {
        let keys = self
            .0
            .keys()
            .filter(|k| k.chars().all(char::is_numeric))
            .count();

        self.0.clear();

        for i in 0..keys {
            self.add_layer(custom_model_data, i);
        }

        keys
    }
}
