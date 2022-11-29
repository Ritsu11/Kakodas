// use gloo_net::http::Request;
// use serde::Deserialize;
// use yew::function_component;
// use yew::prelude::*;

// #[derive(Clone, PartialEq, Deserialize)]
// struct Video {
//     id: usize,
//     title: String,
//     speaker: String,
//     url: String,
// }

// #[function_component(Home)]
// pub fn home() -> Html {
//     let videos = use_state(|| vec![]);
//     {
//         let videos = videos.clone();
//         use_effect_with_deps(
//             move |_| {
//                 wasm_bindgen_futures::spawn_local(async move {
//                     let fetched_videos: Vec<Video> =
//                         Request::get("http://localhost:9000/dreams?user_id=1")
//                             .send()
//                             .await
//                             .unwrap()
//                             .json()
//                             .await
//                             .unwrap();
//                     videos.set(fetched_videos);
//                 });
//                 || ()
//             },
//             (),
//         );
//     }

//     videos
//         .iter()
//         .map(|video| {
//             html! {
//                 <p>{format!("{}: {}", video.speaker, video.title)}</p>
//             }
//         })
//         .collect()
// }
