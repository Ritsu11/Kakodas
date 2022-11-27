use crate::models::state::*;
use crate::models::user::*;
use crate::router::route::Route;
use crate::{models::form::Form, request::request::request_login};
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{events::Event, html, Component, Context, Html, Properties};
use yew_router::prelude::*;

pub struct Login {
    form: Form,
    response: FetchState<String>,
    state: LoginState,
}

pub enum Msg {
    CheckLogin,
    FetchStart,
    InputEmail(String),
    InputPassword(String),
    SendLogin,
    SetFetchState(FetchState<String>),
    SetLoginState(LoginState),
}

#[derive(PartialEq, Properties)]
pub struct Props;

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::CheckLogin);

        Self {
            form: Form {
                email: "".to_string(),
                password: "".to_string(),
            },
            response: FetchState::NotFetching,
            state: LoginState::Failed,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputEmail(arg) => {
                self.form.email = arg.to_string();
                true
            }
            Msg::InputPassword(arg) => {
                self.form.password = arg.to_string();
                true
            }
            Msg::SendLogin => {
                let email = &self.form.email;
                let password = &self.form.password;
                let request = User {
                    id: 1,
                    title: email.clone(),
                    body: password.clone(),
                    userId: 1,
                };

                ctx.link().send_future(async {
                    match request_login("https://jsonplaceholder.typicode.com/posts/1", request)
                        .await
                    {
                        Ok(response) => Msg::SetFetchState(FetchState::Success(response)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                // LocalStorage::set("login", true).ok();
                // LocalStorage::set("id", 10).ok();
                // ctx.link()
                //     .send_message(Msg::SetLoginState(LoginState::Success));
                false
            }
            Msg::SetFetchState(fetch_state) => {
                self.response = fetch_state;
                true
            }
            Msg::FetchStart => {
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
            Msg::SetLoginState(login_state) => {
                self.state = login_state;
                true
            }
            Msg::CheckLogin => {
                let login_status: Option<bool> = LocalStorage::get("login").unwrap_or_default();
                let id_status: Option<i32> = LocalStorage::get("id").unwrap_or_default();

                ctx.link().send_message(match login_status {
                    Some(_) => match id_status {
                        Some(_) => Msg::SetLoginState(LoginState::Success),
                        None => {
                            LocalStorage::delete("login");
                            Msg::SetLoginState(LoginState::Success)
                        }
                    },
                    None => {
                        LocalStorage::delete("id");
                        Msg::SetLoginState(LoginState::Failed)
                    }
                });
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
                FetchState::NotFetching => {
                    let input_email = link.batch_callback(|e: Event| {
                        let target: Option<EventTarget> = e.target();
                        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                        input.map(|input| Msg::InputEmail(input.value()))
                    });
                    let input_password = link.batch_callback(|e: Event| {
                        let target: Option<EventTarget> = e.target();
                        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                        input.map(|input| Msg::InputPassword(input.value()))
                    });

                    html! {
                      <>
                            <div><Link<Route> to={Route::Home}>{ "click here to go Home" }</Link<Route>></div>

                            <div class="wrap">
                                <div class="frame">
                                    <div class="frame_contents">
                                        <div class="logo"><img src="aaa.png" alt="logo"/></div>
                                        <div class="head">{"ログイン"}</div>
                                        <div class="frame_form">
                                            <div class="login_mail">
                                                <p>{"メールアドレス"}</p>
                                                <input type="text" placeholder="Yutaka.FujiFuji@test.com" name="email" onchange={input_email} />
                                            </div>
                                            <div class="login_pass">
                                                <p>{"パスワード"}</p>
                                                <input type="password" placeholder="Password123@" name="psw" onchange={input_password} />
                                            </div>
                                            <input id="login" type="button" value="ログイン" onclick={link.callback(|_| Msg::SendLogin)} />
                                            <div>
                                                <button>{"新規会員登録"}</button>
                                            </div>
                                            <a href="">
                                            <Link<Route> to={Route::Home}>{ "サンプルを見る" }</Link<Route>>
                                            </a>
                                        </div>
                                    </div>
                                </div>
                            </div>
                      </>
                    }
                }
                FetchState::Fetching => html! {<><div>{"Fetching now"}</div></>},
                FetchState::Success(response) => {
                    let json: User = serde_json::from_str(&response).unwrap();
                    html! {
                        <>
                            <Link<Route> to={Route::Home}>{ "click here to go Home" }</Link<Route>>
                            <div>{json.id}</div>
                            <div>{json.title}</div>
                            <div>{json.body}</div>
                            <div>{json.userId}</div>
                        </>
                    }
                }
                FetchState::Failed(err) => html! { err },
            },
        }
    }
}
