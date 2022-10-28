use crate::router::route::Route;

use getrandom::Error;
use gloo::net::http::{Request, Response};
// use reqwest::Request;
use yew::{events::Event, html, Component, Context, Html};
use yew_router::components::Link;

// use gloo::storage::LocalStorage;
// use gloo_storage::Storage;

use serde::{Deserialize, Serialize};
// use wasm_bindgen::JsCast;
// use web_sys::{EventTarget, HtmlInputElement};
// use yew::prelude::*;
// use yew_router::prelude::*;
use js_sys::JsString;
// use web_sys::{console, Request, RequestInit, RequestMode, Response};

pub struct Home;

pub enum Msg {
    Click,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        wasm_bindgen_futures::spawn_local(async {
            let forecast_endpoint = format!(
                "https://api.weather.gov/gridpoints/{office}/{x},{y}/forecast",
                office = "DTX",
                x = 65,
                y = 33
            );
            let resp = Request::get(&forecast_endpoint).send().await.unwrap();
        });

        html! {
            <>
                <button onclick={link.callback(|_| Msg::Click)}>{"show"}</button>
                <div><Link<Route> to={Route::Login}>{ "click here to go Login" }</Link<Route>></div>
                <div><Link<Route> to={Route::Register}>{ "click here to go Register" }</Link<Route>></div>
            </>
        }
    }
}
