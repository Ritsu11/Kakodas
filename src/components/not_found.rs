use yew::prelude::*;

#[function_component(NotFound)]
pub fn header() -> Html {
    html! {
        <div class="wrap">
            <div class="container">
                <div class="boo-wrapper">
                    <div class="boo">
                        <div class="face"></div>
                    </div>
                    <div class="shadow"></div>
                        <h1>{"404"}</h1>
                    <p style="margin-left: 5px;">
                        {"ページが見つかりません"}
                    </p>
                </div>
            </div>
        </div>
    }
}
