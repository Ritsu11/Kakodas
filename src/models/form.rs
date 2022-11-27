use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]

pub struct Form {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct FormDataList(Vec<Form>);
impl Iterator for FormDataList {
    type Item = Form;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
