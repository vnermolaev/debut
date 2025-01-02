use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "bg-gray-100 min-h-screen p-8",
            div { class: "max-w-4xl mx-auto text-center mb-12",
                h1 { class: "text-4xl font-bold text-gray-800", "About Us" }
            }
            // Sections Container
            div { class: "max-w-5xl mx-auto mb-12",
                h2 { class: "text-2xl font-semibold text-blue-600 mb-4", "What is this?" }
                p { class: "text-gray-600 leading-relaxed",
                    "This is a placeholder for a section that describes what this entity, product, or concept is.
                    For example, it could be an application that helps users organize their tasks, a platform for connecting people,
                    or a system to streamline operations. The possibilities are endless!"
                }
            }
            div { class: "max-w-5xl mx-auto",
                h2 { class: "text-2xl font-semibold text-red-600 mb-4", "What is this NOT?" }
                p { class: "text-gray-600 leading-relaxed",
                    "This is a placeholder for a section that clarifies what this entity, product, or concept is not.
                    For example, it is not a magic solution to every problem, not a replacement for professional advice,
                    or not something that can function without user input or engagement."
                }
            }
        }
    }
}