use crate::router::route::Route;

use serde::Deserialize;
use yew::{html, Component, Context, Html, Properties};
use yew_router::components::Link;

use crate::components::home;

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

#[derive(PartialEq, Properties)]
pub struct Props;

pub enum Msg {
    Click,
    Call,
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::Call);

        Self { json: vec![] }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => true,
            Msg::Call => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <>
                <home::Home />

                <button onclick={link.callback(|_| Msg::Click)}>{"show"}</button>
                <div><Link<Route> to={Route::Login}>{ "click here to go Login" }</Link<Route>></div>
                <div><Link<Route> to={Route::Register}>{ "click here to go Register" }</Link<Route>></div>
            </>
        }
    }
}
