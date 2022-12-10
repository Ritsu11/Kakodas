use crate::{models::state::*, models::*, service::request::post_request};
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{events::Event, html, Component, Context, Html};
use yew_router::prelude::*;

use crate::router::route::Route;

pub struct Add {
    form: request::dream_add::DreamAdd,
    response: FetchState<String>,
    state: LoginState,
}

pub enum Msg {
    FetchStart,
    SetFetchState(FetchState<String>),
    CheckLogin,
    SetLoginState(LoginState),
    RequestDreamAdd,
}

impl Component for Add {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::CheckLogin);

        Self {
            form: request::dream_add::DreamAdd {
                user_id: 0,
                title: "".to_string(),
                image_sentence: "".to_string(),
                description: "".to_string(),
                date: "".to_string(),
            },
            response: FetchState::NotFetching,
            state: LoginState::Success,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchStart => {
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
                        None => Msg::SetLoginState(LoginState::Failed),
                    },
                    None => Msg::SetLoginState(LoginState::Failed),
                });
                true
            }
            Msg::SetLoginState(login_state) => {
                self.state = login_state;
                true
            }
            Msg::RequestDreamAdd => {
                log::info!("Update: {:?}", "Click".to_owned());
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
                    <div class="wrap_add">
                        <div class="frame_add">
                            <div class="frame_contents_add">
                                <div class="sakusei_head">{" 夢作成 "}</div>
                                <div class="sakusei_date">
                                    <p>{" 夢を見た日 "}</p>
                                    <input
                                        type="date"
                                        value="2020-01-01"
                                        min="2015-01-01"
                                        max="2040-12-31"
                                    />
                                </div>
                                <div class="sakusei_title">
                                    <p>{" 夢のタイトル "}</p>
                                    <input type="text" placeholder="〇〇の夢" />
                                </div>
                                <div class="sakusei_command">
                                    <p>{" 夢の画像生成文 "}</p>
                                    <textarea  maxlength="100" rows="2" placeholder="最大100文字"></textarea>
                                </div>
                                <div class="sakusei_explain">
                                    <p>{" 夢の説明文 "}</p>
                                    <textarea maxlength="500" rows="5" placeholder="最大100文字"></textarea>
                                </div>
                                <div class="frame_form">
                                    <input id="dream_add" type="button" value="登録" onclick={link.callback(|_| Msg::RequestDreamAdd)} />
                                    <div class="link3">
                                        <Link<Route> to={Route::Home}>{ "ホームに戻る" }</Link<Route>>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    </>
                }
            }
            LoginState::Failed => {
                html! {
                  <>
                    <Redirect<Route> to={Route::Home}/>
                  </>
                }
            }
        }
    }
}
