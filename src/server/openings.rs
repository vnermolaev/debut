use crate::shared::data;
use dioxus::prelude::*;
use tracing::debug;
use crate::shared::data::PlayerColor;

#[server]
pub async fn openings(
    color: PlayerColor,
    start: usize,
    end: Option<usize>,
) -> Result<Vec<data::Opening>, ServerFnError> {
    debug!("Fetching Openings from server for {color}...");

    let openings = match color {
        PlayerColor::White => {
            vec![
                data::Opening {
                    id: 0,
                    color,
                    img_src: "./assets/openings/white/english-opening.png".to_string(),
                    name: "English opening".to_string(),
                },
                data::Opening {
                    id: 1,
                    color,
                    img_src: "./assets/openings/white/italian-game.png".to_string(),
                    name: "Italian game".to_string(),
                },
                data::Opening {
                    id: 2,
                    color,
                    img_src: "./assets/openings/white/queens-gambit.png".to_string(),
                    name: "Queen's gambit".to_string(),
                },
                data::Opening {
                    id: 3,
                    color,
                    img_src: "./assets/openings/white/scotch-game.png".to_string(),
                    name: "Scotch game".to_string(),
                },
            ]
        }
        PlayerColor::Black => {
            vec![
                data::Opening {
                    id: 4,
                    color,
                    img_src: "./assets/openings/black/caro-kann-defence.png".to_string(),
                    name: "Caro-Kann defence".to_string(),
                },
                data::Opening {
                    id: 5,
                    color,
                    img_src: "./assets/openings/black/dutch-defence.png".to_string(),
                    name: "Dutch defence".to_string(),
                },
                data::Opening {
                    id: 6,
                    color,
                    img_src: "./assets/openings/black/french-defence.png".to_string(),
                    name: "French defence".to_string(),
                },
                data::Opening {
                    id: 7,
                    color,
                    img_src: "./assets/openings/black/scandinavian-defence.png".to_string(),
                    name: "Scandinavian defence".to_string(),
                },
                data::Opening {
                    id: 8,
                    color,
                    img_src: "./assets/openings/black/sicilian-defence.png".to_string(),
                    name: "Sicilian defence".to_string(),
                },
            ]
        }
    };

    Ok(openings
        .into_iter()
        .skip(start)
        .take(end.map(|end| end - start).unwrap_or(usize::MAX)) // Take as many as requested or, usize::MAX, i.e., all.
        .collect())
}
