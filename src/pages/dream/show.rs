use crate::{
    models::response::dream::*, models::state::*, router::route::Route,
    service::request::get_request,
};
use gloo::storage::{LocalStorage, Storage};
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
            Msg::FetchStart => {
                ctx.link().send_future(async {
                    let id_state: u32 = LocalStorage::get("id").unwrap();
                    let url = format!("http://localhost:9000/dreams?user_id={id}", id = id_state);

                    match get_request(&url.to_string()).await {
                        Ok(response) => Msg::SetFetchState(FetchState::Success(response)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
            Msg::SetFetchState(fetch_state) => {
                self.response = fetch_state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.response {
            FetchState::NotFetching => html! {<><div>{"Not Fetching"}</div></>},
            FetchState::Fetching => {
                html! {<><div>{"Fetching now"}</div><div class="loader">{"Loading..."}</div></>}
            }
            FetchState::Success(response) => {
                let json: Dreams = serde_json::from_str(&response).unwrap();
                html! {
                    <>
                    <div class="wrap">
                        <div class="header">
                            <img class="logo" src="https://pbs.twimg.com/media/FitbKr5akAAaPBp?format=png&name=360x360" alt="logo" />
                            <div class="header_btn">
                                <div class="sakusei"><Link<Route> to={Route::Home}>{"作成"}</Link<Route>></div>
                                <div class="log-out"><div><Link<Route> to={Route::Login}>{"ログアウト"}</Link<Route>></div></div>
                            </div>
                        </div>
                        <div class="cards_wrap">
                            {
                                json.dreams.map(|data| {
                                    html! {
                                        <>
                                            <div class="card">
                                                <figure>
                                                    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/fe/2004-04-10_Larus_michahellis_ad.jpg/800px-2004-04-10_Larus_michahellis_ad.jpg" alt="" />
                                                    <figcaption>
                                                        <p>{data.date} <br /><strong>{data.title}</strong><br/>{data.description}</p>
                                                    </figcaption>
                                                </figure>
                                            </div>
                                        </>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>
                    </>
                }
            }
            FetchState::Failed(err) => html! { err },
        }
    }
}
