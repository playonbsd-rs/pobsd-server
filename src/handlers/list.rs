use askama::Template;
use axum::{
    extract::{Form, Query, State},
    response::Html,
};
use libpobsd::{Game, Status};
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    app::{pagination::Page, representation::GameRepresentation},
    db::{connector::DbCon, responses::AppDbResponse},
    utils::GameFilterWrapper,
};

use crate::handlers::errors::{ErrorNoGamesTemplate, InternalErrorTemplate};

#[derive(Deserialize, Debug)]
pub struct Search {
    pattern: String,
}

#[derive(Template)]
#[template(path = "game_list.html")]
pub struct GameListTemplate {
    games: Vec<Game>,
    paginator: Page,
    query_str: String,
}

#[derive(Deserialize, Debug)]
pub struct Params {
    pub page: Option<usize>,
    #[serde(flatten)]
    pub filter: GameFilterWrapper,
}

pub async fn game_list(
    State(db_con): State<Arc<DbCon>>,
    Query(params): Query<Params>,
) -> Result<Html<String>, Html<String>> {
    let page = params.page.unwrap_or(1);
    let filter: GameFilterWrapper = params.filter.clone();
    match db_con.get_game_list_paginated_filtered(page, filter).await {
        AppDbResponse::GameListPaginated(games, page) => {
            let gmt = GameListTemplate {
                games,
                paginator: page,
                query_str: params.filter.query_string(),
            };
            Ok(Html(gmt.to_string()))
        }
        AppDbResponse::NoGame => Ok(Html(ErrorNoGamesTemplate {}.to_string())),
        _ => Err(Html(InternalErrorTemplate {}.to_string())),
    }
}

#[derive(Template)]
#[template(path = "news.html")]
struct GameNewTemplate {
    game_representations: Vec<GameRepresentation>,
}

pub async fn news(State(db_con): State<Arc<DbCon>>) -> Result<Html<String>, Html<String>> {
    match db_con.get_recent_games().await {
        AppDbResponse::RecentGames(game_representations) => Ok(Html(
            GameNewTemplate {
                game_representations,
            }
            .to_string(),
        )),
        _ => Err(Html(InternalErrorTemplate {}.to_string())),
    }
}

pub async fn game_list_search(Form(search): Form<Search>) -> axum::response::Redirect {
    let pattern = search.pattern;
    if pattern.is_empty() {
        axum::response::Redirect::to("")
    } else {
        let filter = GameFilterWrapper::from_pattern(&pattern);
        axum::response::Redirect::to(&format!("?{}", &filter.query_string()))
    }
}
