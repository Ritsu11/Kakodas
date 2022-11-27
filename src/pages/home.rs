use crate::models::state::*;
use crate::models::user::UserDataList;
use crate::request::request::fetch_dream;
use crate::router::route::Route;
use yew::{html, Component, Context, Html, Properties};
use yew_router::components::Link;

pub struct Home {
    response: FetchState<String>,
}

pub enum Msg {
    SetFetchState(FetchState<String>),
    FetchStart,
}

#[derive(PartialEq, Properties)]
pub struct Props;

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchStart);

        Self {
            response: FetchState::NotFetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFetchState(fetch_state) => {
                self.response = fetch_state;
                true
            }
            Msg::FetchStart => {
                ctx.link().send_future(async {
                    match fetch_dream("https://jsonplaceholder.typicode.com/posts").await {
                        Ok(response) => Msg::SetFetchState(FetchState::Success(response)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.response {
            FetchState::NotFetching => html! {<><div>{"Not Fetching"}</div></>},
            FetchState::Fetching => html! {<><div>{"Fetching now"}</div></>},
            FetchState::Success(response) => {
                let json: UserDataList = serde_json::from_str(&response).unwrap();
                html! {
                    <>
                        <div><Link<Route> to={Route::DreamShow}>{ "click here to go Dream" }</Link<Route>></div>
                        <div><Link<Route> to={Route::Login}>{ "click here to go Login" }</Link<Route>></div>
                        {
                            json.map(|data| {
                                html! {
                                    <>
                                        <div>{data.userId}</div>
                                        <div>{data.id}</div>
                                        <div>{data.title}</div>
                                        <div>{data.body}</div>
                                    </>
                                }
                            }).collect::<Html>()
                        }

                    </>
                }
            }
            FetchState::Failed(err) => html! { err },
        }
    }
}
