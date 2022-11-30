use crate::models::{response::dreams::Dreams, response::state::*};
use crate::router::route::Route;
use crate::service::request::fetch_dream;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use yew::{html, Component, Context, Html, Properties};
use yew_router::components::Link;

pub struct Home {
    response: FetchState<String>,
}

pub enum Msg {
    SetFetchState(FetchState<String>),
    FetchStart,
    Logout,
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
                    match fetch_dream("http://localhost:9000/dreams?user_id=1").await {
                        Ok(response) => Msg::SetFetchState(FetchState::Success(response)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
            Msg::Logout => {
                LocalStorage::delete("login");
                LocalStorage::delete("id");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let login_state: Option<bool> = LocalStorage::get("login").unwrap_or_default();

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
                                if let Some(true) = login_state {
                                    <div class="sakusei"><Link<Route> to={Route::DreamAdd}>{"作成"}</Link<Route>></div>
                                    <div class="log-out"><a onclick={link.callback(|_| Msg::Logout)}>{"ログアウト"}</a></div>
                                } else {
                                    <div class="sakusei"><Link<Route> to={Route::Register}>{"新規"}</Link<Route>></div>
                                    <div class="log-out"><Link<Route> to={Route::Login}>{"ログイン"}</Link<Route>></div>
                                }
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
                                                    <p>{data.date} <br /><strong>{data.title}</strong><br/>{data.dreamId}</p>
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
