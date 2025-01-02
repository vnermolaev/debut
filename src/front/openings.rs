use crate::front::OpeningCard;
use crate::shared::data;
use dioxus::prelude::*;
use tracing::debug;

#[component]
pub fn Openings(color: String) -> Element {
    let openings = vec![data::Opening {
        color: "White".to_string(),
        img_src: "/assets/openings/white/scotch-game.png".to_string(),
        name: "Scotch game".to_string(),
    }];

    rsx! {
        div {
            class: "bg-gray-100 min-h-screen flex flex-col",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                h2 {
                    class: "text-3xl font-bold text-center text-gray-800 mb-8",
                    "Openings for {color}"
                }
                div {
                    class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8",
                    for opening in openings {
                        OpeningCard { opening },
                    }
                }
            }
        }
    }
}
