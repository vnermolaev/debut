mod client;
mod server;
mod shared;

use crate::client::App;
use dioxus::prelude::*;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    launch(App);
}

// /// Echo component that demonstrates fullstack server functions.
// #[component]
// fn Echo() -> Element {
//     let mut response = use_signal(|| String::new());
//
//     rsx! {
//         div { id: "echo",
//             h4 { "ServerFn Echo" }
//             input {
//                 placeholder: "Type here to echo...",
//                 oninput: move |event| async move {
//                     let data = echo_server(event.value()).await.unwrap();
//                     response.set(data);
//                 },
//             }
//
//             if !response().is_empty() {
//                 p {
//                     "Server echoed: "
//                     i { "{response}" }
//                 }
//             }
//         }
//     }
// }
//
// /// Echo the user input on the server.
// #[server(EchoServer)]
// async fn echo_server(input: String) -> Result<String, ServerFnError> {
//     Ok(input)
// }
