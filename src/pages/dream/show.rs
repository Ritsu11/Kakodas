use crate::models::state::*;
use crate::models::user::User;
use crate::request::request::fetch_dream;
use crate::router::route::Route;
use yew::{html, Component, Context, Html, Properties};
use yew_router::components::Link;

pub struct Show {
    response: FetchState<String>,
}

pub enum Msg {
    SetFetchState(FetchState<String>),
    FetchStart,
}

#[derive(PartialEq, Properties)]
pub struct Props;

impl Component for Show {
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
                    match fetch_dream("https://jsonplaceholder.typicode.com/posts/2").await {
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
            FetchState::NotFetching => html! {<></>},
            FetchState::Fetching => html! {<></>},
            FetchState::Success(response) => {
                let json: User = serde_json::from_str(&response).unwrap();
                html! {
                    <>
                        <Link<Route> to={Route::Home}>{ "click here to go Home" }</Link<Route>>
                        <div>{json.userId}</div>
                        <div>{json.id}</div>
                        <div>{json.title}</div>
                        <div>{json.body}</div>
                    </>
                }
            }
            FetchState::Failed(err) => html! { err },
        }
    }
}
