use crate::models::video::Video;
use gloo_net::{http::Request, Error};

pub async fn request_dream(url: &str) -> Result<Vec<Video>, Error> {
    let fetched_videos: Vec<Video> = Request::get(url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Ok(fetched_videos)
}
