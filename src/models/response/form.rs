use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]

pub struct Form {
    pub id: u32,
    pub login_flg: bool,
    pub status_code: u32,
}
