use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DreamDelete {
    pub dream_id: u128,
}
