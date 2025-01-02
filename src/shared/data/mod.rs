use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct Opening {
    pub color: String,
    pub img_src: String,
    pub name: String,
}