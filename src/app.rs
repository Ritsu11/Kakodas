use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::route::{self};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
             <Switch<route::Route> render={Switch::render(route::switch)} />
        </BrowserRouter>
    }
}
