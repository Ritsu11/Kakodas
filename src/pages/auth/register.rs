use crate::{
    components::not_found::NotFound, models::state::*, models::*, router::route::Route,
    service::request::post_request,
};
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{events::Event, html, Component, Context, Html};
use yew_router::prelude::*;

pub struct Register {
    form: request::form::Form,
    response: FetchState<String>,
    state: LoginState,
}

pub enum Msg {
    CheckLogin,
    SetFetchState(FetchState<String>),
    SetLoginState(LoginState),
    InputEmail(String),
    InputPassword(String),
    RequestRegister,
}

impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::CheckLogin);

        Self {
            form: request::form::Form {
                email: "".to_string(),
                password: "".to_string(),
            },
            response: FetchState::NotFetching,
            state: LoginState::Failed,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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
            Msg::SetFetchState(fetch_state) => {
                self.response = fetch_state;
                true
            }
            Msg::SetLoginState(login_state) => {
                self.state = login_state;
                true
            }
            Msg::InputEmail(arg) => {
                self.form.email = arg.to_string();
                true
            }
            Msg::InputPassword(arg) => {
                self.form.password = arg.to_string();
                true
            }
            Msg::RequestRegister => {
                let email = &self.form.email;
                let password = &self.form.password;
                let request = request::form::Form {
                    email: email.clone(),
                    password: password.clone(),
                };

                ctx.link().send_future(async {
                    match post_request("https://20.63.155.42:9000/users/sign-up", request).await {
                        Ok(response) => Msg::SetFetchState(FetchState::Success(response)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
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
                    <Redirect<Route> to={ Route::Home }/>
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
                        <div class="wrap_login">
                            <div class="frame">
                                <div class="frame_contents">
                                    <div class="logo_login"><img src="https://pbs.twimg.com/media/FitbKr5akAAaPBp?format=png&name=360x360" alt="logo"/></div>
                                    <div class="head">{ "??????????????????" }</div>
                                    <div class="frame_form">
                                        <div class="login_mail">
                                            <p>{ "?????????????????????" }</p>
                                            <input type="text" placeholder="Yutaka.FujiFuji@test.com" name="email" onchange={ input_email } />
                                        </div>
                                        <div class="login_pass">
                                            <p>{ "???????????????" }</p>
                                            <input type="password" placeholder="Password123@" name="psw" onchange={ input_password } />
                                        </div>
                                            <input id="login" type="button" value="??????" onclick={ link.callback(|_| Msg::RequestRegister) } />
                                        <div class="link4">
                                            <Link<Route> to={Route::Home}>{ "?????????????????????" }</Link<Route>>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                      </>
                    }
                }
                FetchState::Fetching => html! {<><div class="loader">{ "Loading..." }</div></>},
                FetchState::Success(response) => {
                    match serde_json::from_str::<response::form::Form>(&response) {
                        Ok(json) => {
                            LocalStorage::set("login", json.login_flg).ok();
                            LocalStorage::set("id", json.id).ok();
                            html! {
                                <Redirect<Route> to={ Route::Home }/>
                            }
                        }
                        Err(_) => {
                            let input_email = link.batch_callback(|e: Event| {
                                let target: Option<EventTarget> = e.target();
                                let input =
                                    target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                                input.map(|input| Msg::InputEmail(input.value()))
                            });
                            let input_password = link.batch_callback(|e: Event| {
                                let target: Option<EventTarget> = e.target();
                                let input =
                                    target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                                input.map(|input| Msg::InputPassword(input.value()))
                            });

                            html! {
                              <>
                                <div class="wrap_login">
                                    <div class="frame">
                                        <div class="frame_contents">
                                            <div class="logo_login"><img src="https://pbs.twimg.com/media/FitbKr5akAAaPBp?format=png&name=360x360" alt="logo"/></div>
                                            <div class="head">{ "??????????????????" }</div>
                                            <div class="frame_form">
                                                <div class="login_mail">
                                                    <p>{ "?????????????????????" }</p>
                                                    <input type="email" placeholder="Yutaka.FujiFuji@test.com" name="email" value={ self.form.email.clone() } onchange={ input_email } />
                                                </div>
                                                <div class="login_pass">
                                                    <p>{ "???????????????" }</p>
                                                    <input type="password" placeholder="Password123@" name="psw" value={ self.form.password.clone() } onchange={ input_password } />
                                                </div>
                                                <div class="login_err">
                                                    <p>{ "??????????????????????????????????????????" }</p>
                                                </div>
                                                    <input id="login" type="button" value="??????" onclick={ link.callback(|_| Msg::RequestRegister) } />
                                                <div class="link4">
                                                    <Link<Route> to={ Route::Home }>{ "?????????????????????" }</Link<Route>>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                              </>
                            }
                        }
                    }
                }
                FetchState::Failed(_) => html! {
                    <>
                        <NotFound />
                    </>
                },
            },
        }
    }
}
