use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DreamEdit {
    pub dream_id: u128,
    pub title: String,
    pub image_sentence: String,
    pub description: String,
    pub date: String,
}
