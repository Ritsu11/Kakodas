use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::auth::*;
use crate::pages::dream::*;
use crate::pages::*;

// ルーティングURL設定
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/dream_add")]
    DreamAdd,
    #[at("/dream_edit/:id")]
    DreamEdit { id: u128 },
    #[at("/dream_show")]
    DreamShow,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// ルーター設定
pub fn switch(routes: Route) -> Html {
    match routes {
        // MainHomePage Routing
        Route::Home => {
            html! {
            <home::Home />
            }
        }

        // Auth Routing
        Route::Login => {
            html! {
                <login::Login />
            }
        }
        Route::Register => {
            html! {
                <register::Register />
            }
        }

        // Dream Routing
        Route::DreamAdd => {
            html! {
                <add::Add />
            }
        }
        Route::DreamEdit { id } => {
            html! {
                <edit::Edit id={id}/>
            }
        }
        Route::DreamShow => {
            html! {
                <show::Show />
            }
        }

        // Error Routing
        Route::NotFound => {
            html! {
                <div class="container">
                    <p>{ "404" }</p>
                </div>
            }
        }
    }
}
