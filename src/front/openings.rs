use dioxus::html::li::value;
use crate::front::OpeningCard;
use crate::shared::data;
use dioxus::prelude::*;
use dioxus_chessboard::PlayerColor;
use tracing::debug;
use crate::front::app::Route;

#[component]
pub fn Openings(color: String, is_subcomponent: bool) -> Element {
    let openings = if color == "white" {
        let mut values = vec![
            data::Opening {
                id: 0,
                color: PlayerColor::White,
                img_src: "/assets/openings/white/english-opening.png".to_string(),
                name: "English opening".to_string(),
            },
            data::Opening {
                id: 1,
                color: PlayerColor::White,
                img_src: "/assets/openings/white/italian-game.png".to_string(),
                name: "Italian game".to_string(),
            },
            data::Opening {
                id: 2,
                color: PlayerColor::White,
                img_src: "/assets/openings/white/queens-gambit.png".to_string(),
                name: "Queen's gambit".to_string(),
            }];

        if !is_subcomponent {
            values.push(
                data::Opening {
                    id: 3,
                    color: PlayerColor::White,
                    img_src: "/assets/openings/white/scotch-game.png".to_string(),
                    name: "Scotch game".to_string(),
                });
        }

        values
    } else {
        let mut values = vec![
            data::Opening {
                id: 4,
                color: PlayerColor::Black,
                img_src: "/assets/openings/black/caro-kann-defence.png".to_string(),
                name: "Caro-Kann defence".to_string(),
            },
            data::Opening {
                id: 5,
                color: PlayerColor::Black,
                img_src: "/assets/openings/black/dutch-defence.png".to_string(),
                name: "Dutch defence".to_string(),
            },
            data::Opening {
                id: 6,
                color: PlayerColor::Black,
                img_src: "/assets/openings/black/french-defence.png".to_string(),
                name: "French defence".to_string(),
            },
            data::Opening {
                id: 7,
                color: PlayerColor::Black,
                img_src: "/assets/openings/black/scandinavian-defence.png".to_string(),
                name: "Scandinavian defence".to_string(),
            }];

        if !is_subcomponent {
            values.push(
                data::Opening {
                    id: 8,
                    color: PlayerColor::Black,
                    img_src: "/assets/openings/black/sicilian-defence.png".to_string(),
                    name: "Sicilian defence".to_string(),
                });
        }

        values
    };

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
