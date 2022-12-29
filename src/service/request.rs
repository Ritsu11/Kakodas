use crate::{
    models::request::form::Form,
    models::request::{dream_add::DreamAdd, dream_delete::DreamDelete, dream_edit::DreamEdit},
    models::state::*,
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestCredentials, RequestInit, RequestMode, Response};

/// ゲットリクエスト
pub async fn get_request(url: &str) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    opts.credentials(RequestCredentials::Include);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = gloo::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    log::info!("Response: {:?}", &text.as_string().unwrap());
    Ok(text.as_string().unwrap())
}

/// ポストリクエスト
pub async fn post_request(url: &str, form: Form) -> Result<String, FetchError> {
    let form = serde_json::to_string(&form).unwrap();
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.credentials(RequestCredentials::Include);
    opts.body(Some(&JsValue::from_str(&form)));
    log::info!("Update: {:?}", &form);

    let request = Request::new_with_str_and_init(url, &opts)?;
    request
        .headers()
        .set("Content-Type", "application/json; charset=UTF-8")?;
    request
        .headers()
        .set("Access-Control-Allow-Credentials", "true")?;

    let window = gloo::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    log::info!("Response: {:?}", &text.as_string().unwrap());
    Ok(text.as_string().unwrap())
}

pub async fn post_dream_request(url: &str, form: DreamAdd) -> Result<String, FetchError> {
    let form = serde_json::to_string(&form).unwrap();
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.credentials(RequestCredentials::Include);
    opts.body(Some(&JsValue::from_str(&form)));
    log::info!("Update: {:?}", &form);

    let request = Request::new_with_str_and_init(url, &opts)?;
    request
        .headers()
        .set("Content-Type", "application/json; charset=UTF-8")?;
    request
        .headers()
        .set("Access-Control-Allow-Credentials", "true")?;

    let window = gloo::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    log::info!("Response: {:?}", &text.as_string().unwrap());
    Ok(text.as_string().unwrap())
}

// プットリクエスト
pub async fn put_dream_request(url: &str, form: DreamEdit) -> Result<String, FetchError> {
    let form = serde_json::to_string(&form).unwrap();
    let mut opts = RequestInit::new();
    opts.method("PUT");
    opts.mode(RequestMode::Cors);
    opts.credentials(RequestCredentials::Include);
    opts.body(Some(&JsValue::from_str(&form)));
    log::info!("Update: {:?}", &form);

    let request = Request::new_with_str_and_init(url, &opts)?;
    request
        .headers()
        .set("Content-Type", "application/json; charset=UTF-8")?;
    request
        .headers()
        .set("Access-Control-Allow-Credentials", "true")?;

    let window = gloo::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    log::info!("Response: {:?}", &text.as_string().unwrap());
    Ok(text.as_string().unwrap())
}

pub async fn delete_dream_request(url: &str, form: DreamDelete) -> Result<String, FetchError> {
    let form = serde_json::to_string(&form).unwrap();
    let mut opts = RequestInit::new();
    opts.method("DELETE");
    opts.mode(RequestMode::Cors);
    opts.credentials(RequestCredentials::Include);
    opts.body(Some(&JsValue::from_str(&form)));
    log::info!("Update: {:?}", &form);

    let request = Request::new_with_str_and_init(url, &opts)?;
    request
        .headers()
        .set("Content-Type", "application/json; charset=UTF-8")?;
    request
        .headers()
        .set("Access-Control-Allow-Credentials", "true")?;

    let window = gloo::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    log::info!("Response: {:?}", &text.as_string().unwrap());
    Ok(text.as_string().unwrap())
}
