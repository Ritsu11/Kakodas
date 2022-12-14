use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub user_id: u32,
}

#[derive(Deserialize, Serialize)]
pub struct UserDataList(Vec<User>);
impl Iterator for UserDataList {
    type Item = User;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
