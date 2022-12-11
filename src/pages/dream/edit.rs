use yew::prelude::*;
use yew_router::components::Link;

use crate::router::route::Route;

pub struct Edit;

impl Component for Edit {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="wrap_edit">
                    <div class="frame_edit">
                        <div class="frame_contents_edit">
                            <div class="henshuu_head">{"編集"}</div>
                            <div class="henshuu_date">
                                <p>{"夢を見た日"}</p>
                                <input type="date" value="2020-01-01" min="2015-01-01" max="2040-12-31" />
                            </div>
                            <div class="henshuu_title">
                                <p>{"夢のタイトル"}</p>
                                <input type="text" placeholder="〇〇の夢" />
                            </div>
                            <div class="henshuu_command">
                                <p>{"夢の画像生成文"}</p>
                                <textarea maxlength="100" rows="2" placeholder="最大100文字"></textarea>
                            </div>
                            <div class="henshuu_explain">
                                <p>{"夢の説明"}{"文"}</p>
                                <textarea maxlength="500" rows="5" placeholder="最大100文字"></textarea>
                            </div>
                            <div class="frame_form">
                                <input id="dream_add" type="button" value="作成" />
                                <div class="link3">
                                    <Link<Route> to={Route::Home}>{ "夢を削除する" }</Link<Route>>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            <Link<Route> to={Route::Home}>{ "click here to go Home" }</Link<Route>>
            </>
        }
    }
}
