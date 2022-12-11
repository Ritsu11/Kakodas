use crate::{
    components::fetch_err::FetchErr, components::not_found::NotFound,
    models::response::dreams::Dreams, models::state::*, router::route::Route,
    service::request::get_request,
};
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use yew::{html, Component, Context, Html};
use yew_router::{components::Link, prelude::Redirect};

pub struct Home {
    response: FetchState<String>,
    state: LoginState,
}

pub enum Msg {
    FetchStart,
    SetFetchState(FetchState<String>),
    CheckLogin,
    SetLoginState(LoginState),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchStart);
        ctx.link().send_message(Msg::CheckLogin);

        Self {
            response: FetchState::NotFetching,
            state: LoginState::Failed,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchStart => {
                ctx.link().send_future(async {
                    match get_request("http://localhost:9000/dreams?user_id=1").await {
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
            Msg::CheckLogin => {
                let login_state: Option<bool> = LocalStorage::get("login").unwrap_or_default();
                let id_state: Option<i32> = LocalStorage::get("id").unwrap_or_default();

                ctx.link().send_message(match login_state {
                    Some(_) => match id_state {
                        Some(_) => Msg::SetLoginState(LoginState::Success),
                        None => {
                            LocalStorage::delete("login");
                            Msg::SetLoginState(LoginState::Failed)
                        }
                    },
                    None => {
                        LocalStorage::delete("id");
                        Msg::SetLoginState(LoginState::Failed)
                    }
                });
                true
            }
            Msg::SetLoginState(login_state) => {
                self.state = login_state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.state {
            LoginState::Success => {
                html! {
                  <>
                    <Redirect<Route> to={Route::DreamShow}/>
                  </>
                }
            }
            LoginState::Failed => match &self.response {
                FetchState::NotFetching => html! {
                    html! {
                        <>
                            <NotFound />
                        </>
                    }
                },
                FetchState::Fetching => {
                    html! {
                        <>
                            <div class="loader">{"Loading..."}</div>
                        </>
                    }
                }
                FetchState::Success(response) => match serde_json::from_str::<Dreams>(&response) {
                    Ok(json) => {
                        html! {
                            <>
                            <div class="wrap">
                                <div class="header">
                                    <img class="logo" src="https://pbs.twimg.com/media/FitbKr5akAAaPBp?format=png&name=360x360" alt="logo" />
                                    <div class="header_btn_login">
                                        <div class="login"><Link<Route> to={Route::Login}>{"ログイン"}</Link<Route>></div>
                                        <div class="registration"><Link<Route> to={Route::Register}>{"新規登録"}</Link<Route>></div>
                                    </div>
                                </div>
                                <div class="cards_wrap">
                                    {
                                        json.dreams.map(|data| {
                                            html! {
                                                <>
                                                    <div class="card">
                                                        <a href="/">
                                                            <figure>
                                                                <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/fe/2004-04-10_Larus_michahellis_ad.jpg/800px-2004-04-10_Larus_michahellis_ad.jpg" alt="" />
                                                                <figcaption>
                                                                    <p>{data.date} <br /><strong>{data.title}</strong><br/>{data.dream_id}</p>
                                                                </figcaption>
                                                            </figure>
                                                        </a>
                                                    </div>
                                                </>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                                <div class="wrap_footer">
                                    <div class="footer">
                                        <div class="footer_txt">
                                        <p>{"Contact　yutfujig08081@gmail.com"}</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            </>
                        }
                    }
                    Err(_) => {
                        html! {
                            <>
                                <FetchErr />
                            </>
                        }
                    }
                },
                FetchState::Failed(_) => {
                    html! {
                        <>
                            <FetchErr />
                        </>
                    }
                }
            },
        }
    }
}
