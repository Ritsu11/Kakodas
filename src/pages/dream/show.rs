use crate::{
    components::fetch_err::FetchErr, components::not_found::NotFound,
    models::response::dreams::Dreams, models::state::*, router::route::Route,
    service::request::get_request,
};
use gloo::storage::{LocalStorage, Storage};
use yew::{html, Component, Context, Html};
use yew_router::{components::Link, prelude::Redirect};

pub struct Show {
    response: FetchState<String>,
    state: LoginState,
}

pub enum Msg {
    FetchStart,
    SetFetchState(FetchState<String>),
    CheckLogin,
    SetLoginState(LoginState),
    Logout,
}

impl Component for Show {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::CheckLogin);

        Self {
            response: FetchState::NotFetching,
            state: LoginState::Success,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchStart => {
                ctx.link().send_future(async {
                    let id_state: Option<i32> = LocalStorage::get("id").unwrap_or_default();

                    let url = format!(
                        "https://20.63.155.42:9000/dreams?user_id={id}",
                        id = id_state.unwrap()
                    );

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
            Msg::CheckLogin => {
                let login_state: Option<bool> = LocalStorage::get("login").unwrap_or_default();
                let id_state: Option<i32> = LocalStorage::get("id").unwrap_or_default();

                ctx.link().send_message(match login_state {
                    Some(_) => match id_state {
                        Some(_) => {
                            ctx.link().send_message(Msg::FetchStart);
                            Msg::SetLoginState(LoginState::Success)
                        }
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
            Msg::Logout => {
                LocalStorage::delete("login");
                LocalStorage::delete("id");
                ctx.link()
                    .send_message(Msg::SetLoginState(LoginState::Failed));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        match &self.state {
            LoginState::Success => match &self.response {
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
                            <div class="loader">{ "Loading..." }</div>
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
                                    <div class="header_btn_logout">
                                        <div class="sakusei"><Link<Route> to={ Route::DreamAdd }>{ "??????" }</Link<Route>></div>
                                        <div class="log-out"><p onclick={ link.callback(|_| Msg::Logout) }>{ "???????????????" }</p></div>
                                    </div>
                                </div>
                                <div class="cards_wrap">
                                    {
                                        json.dreams.map(|data| {
                                            html! {
                                                <>
                                                    <div class="card">
                                                        <Link<Route> to={ Route::DreamEdit { id: data.dreamId } }>
                                                            <figure>
                                                            <img src={ data.image_path } alt="" />
                                                            <figcaption>
                                                                <p>{ data.date } <br /><strong>{ data.title }</strong><br/>{ data.description }</p>
                                                            </figcaption>
                                                            </figure>
                                                        </Link<Route>>
                                                    </div>
                                                </>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                            </div>
                            <div class="wrap_footer">
                                    <div class="footer">
                                        <div class="footer_txt">
                                        <p>{"Contact???yutfujig08081@gmail.com"}</p>
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
            LoginState::Failed => {
                html! {
                  <>
                    <Redirect<Route> to={ Route::Home }/>
                  </>
                }
            }
        }
    }
}
