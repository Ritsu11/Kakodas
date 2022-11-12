use crate::request::{request::request_dream, state::*};
use crate::router::route::Route;

use yew::{html, Component, Context, Html, Properties};
use yew_router::components::Link;

use crate::components::home;
use crate::models::video::Video;

pub struct Home {
    json: Vec<Video>,
}

#[derive(PartialEq, Properties)]
pub struct Props;

impl Component for Home {
    type Message = FetchMessage;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(FetchMessage::FetchStart);

        Self { json: vec![] }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FetchMessage::SetFetchState(fetch_state) => {
                self.response = fetch_state;
                true
            }
            FetchMessage::FetchStart => {
                _ctx.link().send_future(async {
                    match request_dream("/tutorial/data.json").await {
                        Ok(json) => {
                            self.json = json;
                        }
                        Err(err) => {
                            log::error!("{}", err);
                        }
                    }
                });

                _ctx.link()
                    .send_message(FetchMessage::SetFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <>
                // <home::Home />

                // <button onclick={link.callback(|_| Msg::Click)}>{"show"}</button>
                <div><Link<Route> to={Route::Login}>{ "click here to go Login" }</Link<Route>></div>
                <div><Link<Route> to={Route::Register}>{ "click here to go Register" }</Link<Route>></div>
            </>
        }
        // self.iter()
        //     .map(|video| {
        //         html! {
        //             <p>{format!("{}: {}", video.speaker, video.title)}</p>
        //         }
        //     })
        //     .collect()
    }
}
