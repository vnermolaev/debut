use dioxus::prelude::*;
use dioxus_chessboard::{Chessboard, PlayerColor};

#[component]
pub fn Practice(id: usize) -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-100 flex flex-col items-center p-4",
            h2 { class: "text-2xl font-bold text-gray-800 mb-6", "Practice {id}" }
            div { class: "w-2/5",
                Chessboard { color: PlayerColor::White }
            }
        }
    }
}