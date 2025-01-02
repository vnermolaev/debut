use dioxus::prelude::*;
use crate::front::Openings;

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "bg-gray-100 min-h-screen px-4 sm:px-6 lg:px-8",
            div {
                class: "grid grid-cols-1 lg:grid-cols-2 gap-8",
                // Opening for White.
                Openings {
                    color: "White",
                },
                // Opening for Black.
                Openings {
                    color: "Black",
                }
            }
        }
    }
}