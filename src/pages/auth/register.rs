use yew::prelude::*;
use yew_router::components::Link;

use crate::router::route::Route;

pub struct Register;

impl Component for Register {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        // Msg::SubmitLogin => {
        //         wasm_bindgen_futures::spawn_local(async move {
        //             let login_url =
        //                 String::from("https://api.weather.gov/gridpoints/DTX/65,33/forecast");
        //             let fetch_response = Request::get(&login_url)
        //                 .send()
        //                 .await
        //                 .unwrap()
        //                 .text()
        //                 .await
        //                 .unwrap();

        //             console::log_1(&JsString::from(fetch_response));
        //         });

        //         // Set emoji to LocalStorage
        //         self.login = true;
        //         LocalStorage::set("login", self.login.clone()).ok();
        //         self.id += 1;
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Link<Route> to={Route::Home}>{ "click here to go Home" }</Link<Route>>
        }
    }
}
