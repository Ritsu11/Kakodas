use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Form {
    pub email: String,
    pub password: String,
}
