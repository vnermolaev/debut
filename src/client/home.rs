use crate::client::Openings;
use crate::shared::data::PlayerColor;
use dioxus::prelude::*;

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "bg-gray-100 min-h-screen px-4 sm:px-6 lg:px-8",
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8",
                // Opening for White.
                Openings { color: PlayerColor::White, is_subcomponent: true }
                // Opening for Black.
                Openings { color: PlayerColor::Black, is_subcomponent: true }
            }
        }
    }
}
