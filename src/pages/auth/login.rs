use crate::router::route::Route;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew::{events::Event, html, Component, Context, Html};
use yew_router::prelude::*;

// use js_sys::JsString;
// use reqwasm::http::Request;
// use serde::{Deserialize, Serialize};
// use web_sys::console;

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

                let input_email = link.callback(|e: Event| {
                    let target: EventTarget = e
                        .target()
                        .expect("Event should have a target when dispatched");
                    // You must KNOW target is a HtmlInputElement, otherwise
                    // the call to value would be Undefined Behaviour (UB).
                    Msg::InputEmail(target.unchecked_into::<HtmlInputElement>().value())
                });

                html! {
                    <>

                    <h2>{"Login Form"}</h2>

                    <form>

                        <div class="container">
                            <label for="uname"><b>{"e-mail"}</b></label>
                            <input type="text" name="email"  value={self.email.clone()} oninput={link.callback(|e:InputData| Msg::SetTitle(e.value))}/>

                            <label for="psw"><b>{"Password"}</b></label>
                            <input type="password" placeholder="Enter Password" name="psw" />

                            <button type="submit">{"Login"}</button>
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
                        <button onclick={link.callback(|_| Msg::SubmitLogin)}>{"+1"}</button>
                    </div>

                    </>
                }
            }
        }
    }
}
