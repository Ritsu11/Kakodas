// use crate::router::route::Route;

use gloo_net::http::Request;
use serde::Deserialize;
use yew::{html, Component, Context, Html};
// use yew_router::components::Link;

// use crate::components::home;

#[derive(Clone)]
pub struct Home {
    json: Vec<Video>,
}

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

pub enum Msg {
    Click,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut data: Vec<Video> = Vec::new();
        {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                for video in fetched_videos {
                    &mut data.push(video.clone());
                }
            });
        }

        Self { json: vec![] }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => {
                // console::log_1(&JsString::from("hello".to_string()));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        self.json
            .iter()
            .map(|video| {
                html! {
                    <p>{format!("{}: {}", video.speaker, video.title)}</p>
                }
            })
            .collect()

        // html! {
        //     <>
        //         // <home::Home />

        //         <button onclick={link.callback(|_| Msg::Click)}>{"show"}</button>
        //         <div><Link<Route> to={Route::Login}>{ "click here to go Login" }</Link<Route>></div>
        //         <div><Link<Route> to={Route::Register}>{ "click here to go Register" }</Link<Route>></div>
        //     </>
        // }
    }
}
