use crate::models::{form::Form, state::FetchError, user::*};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// Get Requests
/// 夢記録取得リクエスト
pub async fn fetch_dream(url: &str) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = gloo::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    log::info!("Response: {:?}", &text.as_string().unwrap());
    Ok(text.as_string().unwrap())
}

/// ユーザー情報取得リクエスト
pub async fn fetch_user(url: &str) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::NoCors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = gloo::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    log::info!("Response: {:?}", &text.as_string().unwrap());
    Ok(text.as_string().unwrap())
}

/// Post Request
/// ログインリクエスト
pub async fn request_login(url: &str, form: User) -> Result<String, FetchError> {
    let form = serde_json::to_string(&form).unwrap();
    let mut opts = RequestInit::new();
    opts.method("PUT");
    opts.body(Some(&JsValue::from_str(&form)));
    log::info!("Update: {:?}", &form);

    let request = Request::new_with_str_and_init(url, &opts)?;
    request
        .headers()
        .set("Content-Type", "application/json; charset=UTF-8")?;
    request
        .headers()
        .set("Set-Cookie", "SameSite=None; Secure")?;

    let window = gloo::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    log::info!("Reponse: {:?}", &text.as_string().unwrap());
    Ok(text.as_string().unwrap())
}
