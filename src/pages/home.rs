use yew::prelude::*;
use yew_router::components::Link;

use crate::router::route::Route;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div><Link<Route> to={Route::Login}>{ "click here to go Login" }</Link<Route>></div>
                <div><Link<Route> to={Route::Register}>{ "click here to go Register" }</Link<Route>></div>
            </>
        }
    }
}
