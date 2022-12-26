#![allow(non_snake_case)]

mod app;
mod components;
mod models;
mod pages;
mod router;
mod service;

// アプリのエントリーポイント
fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<app::App>::new().render();
}
