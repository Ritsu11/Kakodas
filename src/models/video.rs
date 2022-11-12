use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}
