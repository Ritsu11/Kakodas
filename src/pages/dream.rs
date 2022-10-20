use yew::prelude::*;
use yew_router::components::Link;

use crate::router::route::Route;

pub struct Dream;

impl Component for Dream {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Link<Route> to={Route::Home}>{ "click here to go Home" }</Link<Route>>
        }
    }
}
