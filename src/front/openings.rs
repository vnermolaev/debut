use crate::front::OpeningCard;
use crate::shared::data;
use dioxus::prelude::*;
use tracing::debug;

#[component]
pub fn Openings(color: String) -> Element {
    let openings = if color == "White" {
        vec![
            data::Opening {
                color: "White".to_string(),
                img_src: "/assets/openings/white/english-opening.png".to_string(),
                name: "English opening".to_string(),
            },
            data::Opening {
                color: "White".to_string(),
                img_src: "/assets/openings/white/italian-game.png".to_string(),
                name: "Italian game".to_string(),
            },
            data::Opening {
                color: "White".to_string(),
                img_src: "/assets/openings/white/queens-gambit.png".to_string(),
                name: "Queen's gambit".to_string(),
            },
            data::Opening {
                color: "White".to_string(),
                img_src: "/assets/openings/white/scotch-game.png".to_string(),
                name: "Scotch game".to_string(),
            },
        ]
    } else {
        vec![
            data::Opening {
                color: "Black".to_string(),
                img_src: "/assets/openings/black/caro-kann-defence.png".to_string(),
                name: "Caro-Kann defence".to_string(),
            },
            data::Opening {
                color: "Black".to_string(),
                img_src: "/assets/openings/black/dutch-defence.png".to_string(),
                name: "Dutch defence".to_string(),
            },
            data::Opening {
                color: "Black".to_string(),
                img_src: "/assets/openings/black/french-defence.png".to_string(),
                name: "French defence".to_string(),
            },
            data::Opening {
                color: "Black".to_string(),
                img_src: "/assets/openings/black/scandinavian-defence.png".to_string(),
                name: "Scandinavian defence".to_string(),
            },
            data::Opening {
                color: "Black".to_string(),
                img_src: "/assets/openings/black/sicilian-defence.png".to_string(),
                name: "Sicilian defence".to_string(),
            },
        ]
    };

    rsx! {
        div {
            class: "border border-gray-300 bg-gray-100 min-h-screen flex flex-col",
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
