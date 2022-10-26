use crate::router::route::Route;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{events::Event, html, Component, Context, Html};
use yew_router::prelude::*;

use js_sys::JsString;
use web_sys::console;

pub struct Login {
    pub email: String,
    pub password: String,
}

pub enum Msg {
    InputEmail(String),
    InputPassword(String),
    SubmitLogin,
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            email: "".to_string(),
            password: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputEmail(str) => {
                self.email = str.to_string();
                console::log_1(&JsString::from(self.email.to_string()));
                true
            }
            Msg::InputPassword(str) => {
                self.password = str.to_string();
                console::log_1(&JsString::from(self.password.to_string()));
                true
            }
            Msg::SubmitLogin => {
                wasm_bindgen_futures::spawn_local(async move {
                    let _login_url =
                        String::from("https://api.weather.gov/gridpoints/DTX/65,33/forecast");

                    // let fetch_response = Request::post(&login_url)
                    //     .header(ContentType::json())
                    //     .body(json_request)
                    //     .send()
                    //     .await
                    //     .unwrap()
                    //     .text()
                    //     .await
                    //     .unwrap();
                });

                // Set login to LocalStorage
                LocalStorage::set("login", true).ok();
                LocalStorage::set("id", 10).ok();
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let login_status: Option<bool> = LocalStorage::get("login").unwrap_or_default();
        let id_status: Option<i32> = LocalStorage::get("id").unwrap_or_default();

        match login_status {
            Some(_a) => match id_status {
                Some(_i) => html! {
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

                            // <button type="submit">{"Login"}</button>
                            <button onclick={link.callback(|_| Msg::SubmitLogin)}>{"ログイン"}</button>
                            <label>
                            <input type="checkbox" name="remember" />
                            </label>
                        </div>

                        <div class="container" style="background-color: #f1f1f1">
                            <button type="button" class="cancelbtn" >{"Cancel"}</button>
                            <span class="psw">{"Forgot"} <a href="/">{"password?"}</a></span>
                        </div>
                    </form>

                    <div>
                    </div>

                    </>
                }
            }
        }
    }
}
