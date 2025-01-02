use dioxus::prelude::*;
use dioxus_chessboard::PlayerColor;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct Opening {
    pub id: usize,
    pub color: PlayerColor,
    pub name: String,
    pub img_src: String,

}
