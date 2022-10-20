use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::dream::Dream;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::user::User;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/user")]
    User,
    #[at("dream")]
    Dream,
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! {
                <Home />
            }
        }
        Route::Login => {
            html! {
                <Login />
            }
        }
        Route::User => {
            html! {
                <User />
            }
        }
        Route::Dream => {
            html! {
                <Dream />
            }
        }
        Route::NotFound => {
            html! {
                <div class="container">
                    <p>{ "404" }</p>
                </div>
            }
        }
    }
}
