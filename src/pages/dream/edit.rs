use crate::{
    components::fetch_err::FetchErr,
    components::not_found::NotFound,
    models::request::{dream_delete::DreamDelete, dream_edit::DreamEdit},
    models::response::dream::*,
    models::state::*,
    router::route::Route,
    service::request::{delete_dream_request, get_request, put_dream_request},
};
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{events::Event, html, Component, Context, Html, Properties};
use yew_router::prelude::*;

pub struct Edit {
    form: DreamEdit,
    response: FetchState<String>,
    state: LoginState,
    dream_id: u128,
}

pub enum Msg {
    FetchStart,
    SetFetchState(FetchState<String>),
    CheckLogin,
    SetLoginState(LoginState),
    InputDate(String),
    InputTitle(String),
    InputImageDescription(String),
    RequestDreamEdit,
    RequestDreamDelete,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u128,
}

impl Component for Edit {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::CheckLogin);
        ctx.link().send_message(Msg::FetchStart);

        Self {
            form: DreamEdit {
                dream_id: 0,
                title: "".to_string(),
                image_sentence: "".to_string(),
                description: "".to_string(),
                date: "".to_string(),
            },
            response: FetchState::NotFetching,
            state: LoginState::Success,
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
                        "https://20.63.155.42:9000/dreams/reading?dream_id={id}",
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
            Msg::InputDate(arg) => {
                self.form.date = arg;
                true
            }
            Msg::InputTitle(arg) => {
                self.form.title = arg;
                true
            }
            Msg::InputImageDescription(arg) => {
                self.form.description = arg;
                true
            }
            Msg::RequestDreamEdit => {
                let date = &self.form.date;
                let title = &self.form.title;
                let image_sentence = &self.form.image_sentence;
                let description = &self.form.description;
                let request = DreamEdit {
                    dream_id: self.dream_id.clone(),
                    title: title.clone(),
                    image_sentence: image_sentence.clone(),
                    description: description.clone(),
                    date: date.clone(),
                };

                ctx.link().send_future(async {
                    match put_dream_request("https://20.63.155.42:9000/dreams/edit", request).await
                    {
                        Ok(_) => Msg::SetLoginState(LoginState::Failed),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                true
            }
            Msg::RequestDreamDelete => {
                let request = DreamDelete {
                    dream_id: LocalStorage::get("dream_id").unwrap(),
                };

                ctx.link().send_future(async {
                    match delete_dream_request("https://20.63.155.42:9000/dreams/delete", request)
                        .await
                    {
                        Ok(response) => Msg::SetFetchState(FetchState::Success(response)),
                        Err(err) => {
                            log::info!("Update: {:?}", &err);
                            Msg::SetFetchState(FetchState::Failed(err))
                        }
                    }
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        match &self.state {
            LoginState::Failed => {
                html! {
                  <>
                    <Redirect<Route> to={Route::Home}/>
                  </>
                }
            }
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
                            <div class="loader">{"Loading..."}</div>
                        </>
                    }
                }
                FetchState::Success(response) => match serde_json::from_str::<Dreams>(&response) {
                    Ok(json) => {
                        html! {
                            <>
                            <div>
                                <div class="wrap_edit">
                                    <div class="frame_edit">
                                        <div class="frame_contents_edit">
                                            <div class="henshuu_head">{"??????"}</div>
                                                {
                                                    json.dream.map(|data| {
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
                                                        let input_description = link.batch_callback(|e: Event| {
                                                            let target: Option<EventTarget> = e.target();
                                                            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                                                            input.map(|input| Msg::InputImageDescription(input.value()))
                                                        });

                                                        html! {
                                                            <>
                                                                <div class="henshuu_date">
                                                                    <p>{"???????????????"}</p>
                                                                    <input type="date" min="2015-01-01" max="2040-12-31" placeholder={ data.date } onchange={ input_date } />
                                                                </div>
                                                                <div class="henshuu_title">
                                                                    <p>{"??????????????????"}</p>
                                                                    <input type="text" placeholder={ data.title.clone() } onchange={ input_title } />
                                                                </div>
                                                                <div class="henshuu_explain">
                                                                    <p>{"????????????"}{"???"}</p>
                                                                    <input type="text" placeholder={ data.description } onchange={ input_description } />
                                                                </div>
                                                            </>
                                                        }
                                                    }).collect::<Html>()
                                                }
                                                <div class="frame_form">
                                                <input id="dream_add" type="button" value="??????" onclick={ link.callback(|_| Msg::RequestDreamEdit) } />
                                                <div class="link3">
                                                    // <Link<Route> to={Route::Home} >{ "??????????????????" }</Link<Route>>
                                                    <a href="/" onclick={ link.callback(|_| Msg::RequestDreamDelete) }>{ "??????????????????" }</a>
                                                </div>
                                            </div>
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
