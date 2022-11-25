use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub userId: u32,
    pub id: u32,
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserDataList(Vec<User>);
impl Iterator for UserDataList {
    type Item = User;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
