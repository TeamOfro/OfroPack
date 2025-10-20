use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {
    pub animation: AnimationData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationData {
    pub frametime: u32,
}
