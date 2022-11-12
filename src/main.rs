#![allow(non_snake_case)]
mod app;
mod components;
mod models;
mod pages;
mod request;
mod router;

// App Boot
fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<app::App>();
}
