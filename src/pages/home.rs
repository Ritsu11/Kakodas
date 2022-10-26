use crate::router::route::Route;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew::{events::Event, html, Component, Context, Html};
use yew_router::components::Link;
use yew_router::prelude::*;

use js_sys::JsString;
use web_sys::console;

pub struct Home;

pub enum Msg {
    Click,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        wasm_bindgen_futures::spawn_local(async move {
            let get_dream = format!(
                "https://api.weather.gov/gridpoints/{office}/{x},{y}/forecast",
                office = "DTX",
                x = 65,
                y = 33
            );

            let fetch_response = Request::get(&get_dream)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            console::log_1(&JsString::from(fetch_response));
        });
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <>
                <button onclick={link.callback(|_| Msg::Click)}>{"show"}</button>
                <div><Link<Route> to={Route::Login}>{ "click here to go Login" }</Link<Route>></div>
                <div><Link<Route> to={Route::Register}>{ "click here to go Register" }</Link<Route>></div>
            </>
        }
    }
}
