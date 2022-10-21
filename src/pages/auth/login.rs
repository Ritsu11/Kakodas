use yew::prelude::*;
use yew_router::components::Link;

use crate::router::route::Route;

pub struct Login;

impl Component for Login {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>

            <h2>{"Login Form"}</h2>

            <form action="/" method="post">

                <div class="container">
                    <label for="uname"><b>{"Username"}</b></label>
                    <input type="text" placeholder="Enter Username" name="uname" />

                    <label for="psw"><b>{"Password"}</b></label>
                    <input
                    type="password"
                    placeholder="Enter Password"
                    name="psw"
                    />

                    <button type="submit">{"Login"}</button>
                    <label>
                    <input type="checkbox" name="remember" />
                    </label>
                </div>

                <div class="container" style="background-color: #f1f1f1">
                    <button type="button" class="cancelbtn">{"Cancel"}</button>
                    <span class="psw">{"Forgot"} <a href="#">{"password?"}</a></span>
                </div>
            </form>

             <Link<Route> to={Route::Home}>{ "click here to go Home" }</Link<Route>>

            </>
        }
    }
}
