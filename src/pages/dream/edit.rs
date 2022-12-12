use crate::{
    components::fetch_err::FetchErr, components::not_found::NotFound, models::response::dream::*,
    models::state::*, router::route::Route, service::request::get_request,
};
use gloo::storage::{LocalStorage, Storage};
use yew::{html, Component, Context, Html, Properties};
use yew_router::{components::Link, prelude::Redirect};

pub struct Edit {
    response: FetchState<String>,
    state: LoginState,
    dream_id: u128,
}

pub enum Msg {
    FetchStart,
    SetFetchState(FetchState<String>),
    CheckLogin,
    SetLoginState(LoginState),
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u128,
}

impl Component for Edit {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchStart);

        Self {
            response: FetchState::NotFetching,
            state: LoginState::Failed,
            dream_id: ctx.props().id,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchStart => {
                LocalStorage::set("dream_id", self.dream_id.clone());

                ctx.link().send_future(async {
                    let id_state: u32 = LocalStorage::get("dream_id").unwrap();
                    let url = format!(
                        "http://localhost:9000/dreams/reading?dream_id={id}",
                        id = id_state
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        match &self.state {
            LoginState::Success => {
                html! {
                  <>
                    <Redirect<Route> to={Route::Home}/>
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
                FetchState::Success(response) => {
                    let json: Dreams = serde_json::from_str(&response).unwrap();
                    html! {
                        <>
                        <div>
                            <div class="wrap_edit">
                                <div class="frame_edit">
                                    <div class="frame_contents_edit">
                                        <div class="henshuu_head">{"編集"}</div>
                                            {
                                                json.dream.map(|data| {
                                                    html! {
                                                        <>
                                                            <div class="henshuu_date">
                                                                <p>{"夢を見た日"}</p>
                                                                <input type="date" value="2020-01-01" min="2015-01-01" max="2040-12-31" value={ data.date } />
                                                            </div>
                                                            <div class="henshuu_title">
                                                                <p>{"夢のタイトル"}</p>
                                                                <input type="text" placeholder="〇〇の夢" value={ data.title }/>
                                                            </div>
                                                            <div class="henshuu_explain">
                                                                <p>{"夢の説明"}{"文"}</p>
                                                                <textarea maxlength="500" rows="5" placeholder="最大100文字" value={ data.description }></textarea>
                                                            </div>
                                                        </>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        <div class="frame_form">
                                            <input id="dream_add" type="button" value="作成" />
                                            <div class="link3">
                                                <Link<Route> to={Route::Home}>{ "夢を削除する" }</Link<Route>>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        </>
                    }
                }
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
