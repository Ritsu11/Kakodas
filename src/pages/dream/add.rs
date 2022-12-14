use crate::{
    components::fetch_err::FetchErr, models::request::dream_add::DreamAdd, models::state::*,
    router::route::Route, service::request::post_dream_request,
};
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{events::Event, html, Component, Context, Html};
use yew_router::prelude::*;

pub struct Add {
    form: DreamAdd,
    response: FetchState<String>,
    state: LoginState,
}

pub enum Msg {
    CheckLogin,
    SetFetchState(FetchState<String>),
    SetLoginState(LoginState),
    InputDate(String),
    InputTitle(String),
    InputImageSentence(String),
    InputImageDescription(String),
    RequestDreamAdd,
}

impl Component for Add {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::CheckLogin);

        Self {
            form: DreamAdd {
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
            Msg::InputDate(arg) => {
                self.form.date = arg;
                true
            }
            Msg::InputTitle(arg) => {
                self.form.title = arg;
                true
            }
            Msg::InputImageSentence(arg) => {
                self.form.image_sentence = arg;
                true
            }
            Msg::InputImageDescription(arg) => {
                self.form.description = arg;
                true
            }
            Msg::RequestDreamAdd => {
                let user_id = LocalStorage::get("id").unwrap();
                let date = &self.form.date;
                let title = &self.form.title;
                let image_sentence = &self.form.image_sentence;
                let description = &self.form.description;
                let request = DreamAdd {
                    user_id: user_id,
                    title: title.clone(),
                    image_sentence: image_sentence.clone(),
                    description: description.clone(),
                    date: date.clone(),
                };

                ctx.link().send_future(async {
                    match post_dream_request("https://20.63.155.42:9000/dreams/register", request)
                        .await
                    {
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
            LoginState::Success => match &self.response {
                FetchState::NotFetching => {
                    let input_date = link.batch_callback(|e: Event| {
                        let target: Option<EventTarget> = e.target();
                        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                        input.map(|input| Msg::InputDate(input.value()))
                    });
                    let input_title = link.batch_callback(|e: Event| {
                        let target: Option<EventTarget> = e.target();
                        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                        input.map(|input| Msg::InputTitle(input.value()))
                    });
                    let input_image_sentence = link.batch_callback(|e: Event| {
                        let target: Option<EventTarget> = e.target();
                        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                        input.map(|input| Msg::InputImageSentence(input.value()))
                    });
                    let input_description = link.batch_callback(|e: Event| {
                        let target: Option<EventTarget> = e.target();
                        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                        input.map(|input| Msg::InputImageDescription(input.value()))
                    });

                    html! {
                        <>
                        <div class="wrap_add">
                            <div class="frame_add">
                                <div class="frame_contents_add">
                                    <div class="sakusei_head">{" ????????? "}</div>
                                    <div class="sakusei_date">
                                        <p>{ "???????????????" }</p>
                                        <input type="date" min="2015-01-01" max="2040-12-31" onchange={ input_date } />
                                    </div>
                                    <div class="sakusei_title">
                                        <p>{ "??????????????????" }</p>
                                        <input type="text" placeholder="????????????" onchange={ input_title } />
                                    </div>
                                    <div class="sakusei_command">
                                        <p>{ "?????????????????????" }</p>
                                        <input type="text" placeholder="??????100??????" onchange={ input_image_sentence } />
                                    </div>
                                    <div class="sakusei_explain">
                                        <p>{ "???????????????" }</p>
                                        <input type="text" placeholder="??????500??????" onchange={ input_description } />
                                    </div>
                                    <div class="frame_form">
                                        <input id="dream_add" type="button" value="??????" onclick={ link.callback(|_| Msg::RequestDreamAdd) } />
                                        <div class="link3">
                                            <Link<Route> to={ Route::Home }>{ "??????????????????" }</Link<Route>>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        </>
                    }
                }
                FetchState::Fetching => {
                    html! {
                        <>
                            <div class="loader">{"Loading..."}</div>
                        </>
                    }
                }
                FetchState::Success(_) => {
                    html! {
                      <>
                        <Redirect<Route> to={ Route::Home }/>
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
