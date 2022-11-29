use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Dreams {
    pub dreams: DreamDataList,
    pub status_code: u32,
}

#[derive(Deserialize, Serialize, Debug)]

pub struct Dream {
    pub date: String,
    pub dreamId: u32,
    pub title: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DreamDataList(Vec<Dream>);

impl Iterator for DreamDataList {
    type Item = Dream;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}