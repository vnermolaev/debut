use crate::shared::data;
use dioxus::prelude::*;

#[component]
pub fn OpeningCard(opening: data::Opening) -> Element {
    rsx! {
        article {
            class: "max-w-sm bg-white rounded-lg shadow-md overflow-hidden hover:shadow-xl hover:ring-2 hover:ring-blue-400 transition-all duration-300",
            div {
                class: "p-4",
                a {
                    href: "#",
                    h3 {
                        class: "text-lg font-semibold text-gray-800",
                        "{opening.name}"
                    }
                }
            }
            img {
                src: "{opening.img_src}",
                class: "w-full h-48 object-contain bg-gray-100",
            },
        }
    }
}
