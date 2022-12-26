use yew::prelude::*;

#[function_component(FetchErr)]
pub fn fetch_err() -> Html {
    html! {
        <div>
            <div class="container">
                <div class="boo-wrapper">
                    <div class="boo">
                        <div class="face"></div>
                    </div>
                    <div class="shadow"></div>
                        <h1>{"FetchError"}</h1>
                    <p style="margin-left: 5px;">
                        {"内容が見つかりません"}
                    </p>
                </div>
            </div>
        </div>
    }
}
