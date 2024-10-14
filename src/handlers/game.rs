use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{rejection::PathRejection, Path, State},
    response::Html,
};
// Status is used in template
use libpobsd::{Game, Status};

use crate::{
    app::representation::GameRepresentation,
    db::{connector::DbCon, data::GameMetaData, responses::AppDbResponse},
    handlers::errors::ErrorNoGameTemplate,
};

#[derive(Template)]
#[template(path = "game_details.html")]
pub struct GameDetailsTemplate {
    game: Game,
    metadata: Option<GameMetaData>,
}

impl GameDetailsTemplate {
    fn new(game_representation: GameRepresentation) -> GameDetailsTemplate {
        GameDetailsTemplate {
            game: game_representation.game,
            metadata: game_representation.metadata,
        }
    }
}

pub async fn game_details(
    State(db_con): State<Arc<DbCon>>,
    game_id: Result<Path<u32>, PathRejection>,
) -> Result<Html<String>, Html<String>> {
    let game_id = match game_id {
        Ok(Path(game_id)) => game_id,
        Err(_) => return Err(Html(ErrorNoGameTemplate {}.to_string())),
    };
    match db_con.get_game_representation(game_id).await {
        AppDbResponse::GameRepresentation(game_representation) => {
            let body = GameDetailsTemplate::new(game_representation);
            Ok(Html(body.to_string()))
        }
        AppDbResponse::Pending(rx) => match rx.await {
            Ok(metadata) => {
                db_con.insert_metadata(game_id, metadata).await;
                match db_con.get_game_representation(game_id).await {
                    AppDbResponse::GameRepresentation(game_representation) => {
                        let body = GameDetailsTemplate::new(game_representation);
                        Ok(Html(body.to_string()))
                    }
                    _ => Err(Html(ErrorNoGameTemplate {}.to_string())),
                }
            }
            _ => Err(Html(ErrorNoGameTemplate {}.to_string())),
        },
        _ => Err(Html(ErrorNoGameTemplate {}.to_string())),
    }
}

pub async fn game_details_random(
    State(db_con): State<Arc<DbCon>>,
) -> Result<axum::response::Redirect, Html<String>> {
    match db_con.get_random_uid().await {
        AppDbResponse::RandomUid(uid) => Ok(axum::response::Redirect::to(&format!("{}", uid))),
        _ => Err(Html(ErrorNoGameTemplate {}.to_string())),
    }
}
