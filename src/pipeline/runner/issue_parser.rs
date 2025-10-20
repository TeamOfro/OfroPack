use crate::schema::animation::AnimationInfo;

#[derive(Debug)]
pub enum ParsedIssue {
    AddCustomModel {
        materials: Vec<String>,
        custom_model_data: String,
        image_url: String,
        animation: Option<AnimationInfo>,
    },
    Add3DCustomModel {
        materials: Vec<String>,
        custom_model_data: String,
        model_json: String,
        layere_image_urls: Vec<String>,
    },
    Extend {
        materials: Vec<String>,
        custom_model_data: String,
    },
}
