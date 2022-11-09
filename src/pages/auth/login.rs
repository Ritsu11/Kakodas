use crate::router::route::Route;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{events::Event, html, Component, Context, Html};
use yew_router::prelude::*;

pub struct Login {
    pub email: String,
    pub password: String,
}

pub enum Msg {
    InputEmail(String),
    InputPassword(String),
    SubmitLogin,
    Call,
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        _ctx.link().send_message(Msg::Call);

        Self {
            email: "".to_string(),
            password: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputEmail(str) => {
                self.email = str.to_string();
                true
            }
            Msg::InputPassword(str) => {
                self.password = str.to_string();
                true
            }
            Msg::SubmitLogin => {
                // Set login to LocalStorage
                LocalStorage::set("login", true).ok();
                LocalStorage::set("id", 10).ok();
                true
            }
            Msg::Call => {
                self.email = "check ctx load".to_string();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        // Fetch LocalStorage Data
        let login_status: Option<bool> = LocalStorage::get("login").unwrap_or_default();
        let id_status: Option<i32> = LocalStorage::get("id").unwrap_or_default();

        // Check LocalStorage LoginData
        match login_status {
            Some(_login) => match id_status {
                Some(_id) => html! {
                    <Redirect<Route> to={Route::Home}/>
                },
                None => {
                    LocalStorage::delete("login");

                    html! {
                        <Redirect<Route> to={Route::Home}/>
                    }
                }
            },
            None => {
                LocalStorage::delete("id");

                // Callback detect input form
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

                    <h2>{"Login Form"}</h2>

                    <form>
                        <div class="container">
                            <label for="uname"><b>{"e-mail"}</b></label>
                            <input type="text" placeholder="Enter Email" name="email" onchange={input_email} />

                            <label for="psw"><b>{"Password"}</b></label>
                            <input type="password" placeholder="Enter Password" name="psw" onchange={input_password}/>

                            <button onclick={link.callback(|_| Msg::SubmitLogin)}>{"ログイン"}</button>
                            <label>
                            </label>
                        </div>
                    </form>

                    <div>
                        { self.email.to_string() }
                    </div>
                    <div>
                        { self.password.to_string() }
                    </div>

                    </>
                }
            }
        }
    }
}
