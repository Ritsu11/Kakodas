use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Dream {
    pub user_id: u32,
    pub title: String,
    pub image_sentence: String,
    pub description: String,
    pub date: String,
}
