use crate::client::app::Route;
use crate::client::OpeningCard;
use crate::server;
use crate::shared::data::PlayerColor;
use dioxus::prelude::*;
use thiserror::Error;
use tracing::debug;

#[component]
pub fn Openings(color: PlayerColor, is_subcomponent: bool) -> Element {
    let openings = use_server_future(move || async move {
        server::openings(color, 0, is_subcomponent.then_some(3))
            .await
            .unwrap()
    })?()
        .ok_or(RenderError::from(OpeningsError::FetchOpeningsFailed))?;

    rsx! {
        div { class: "border border-gray-300 bg-gray-100 min-h-screen flex flex-col",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                h2 { class: "text-3xl font-bold text-center text-gray-800 mb-8",
                    "Openings for {color}"
                }
                div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8",
                    for opening in openings {
                        OpeningCard { opening }
                    }
                }
                if is_subcomponent {
                    div { class: "mt-8 text-right",
                        Link {
                            class: "text-blue-600 font-medium hover:text-blue-800 transition-colors duration-200",
                            to: Route::Openings {
                                color,
                                is_subcomponent: false,
                            },
                            "More..."
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Error)]
enum OpeningsError {
    #[error("Failed to fetch openings")]
    FetchOpeningsFailed,
}
