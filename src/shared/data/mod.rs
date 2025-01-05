use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize)]
pub struct Opening {
    pub id: usize,
    pub color: PlayerColor,
    pub name: String,
    pub img_src: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PlayerColor {
    White,
    Black,
}

impl Display for PlayerColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerColor::White => write!(f, "White"),
            PlayerColor::Black => write!(f, "Black"),
        }
    }
}

impl TryFrom<&str> for PlayerColor {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.to_lowercase() == "white" {
            Ok(PlayerColor::White)
        } else if s.to_lowercase() == "black" {
            Ok(PlayerColor::Black)
        } else {
            Err(format!("Invalid player color: {s}"))
        }
    }
}

impl FromStr for PlayerColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<PlayerColor> for dioxus_chessboard::PlayerColor {
    fn from(color: PlayerColor) -> Self {
        match color {
            PlayerColor::White => dioxus_chessboard::PlayerColor::White,
            PlayerColor::Black => dioxus_chessboard::PlayerColor::Black,
        }
    }
}
