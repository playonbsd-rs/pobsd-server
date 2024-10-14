use askama::Template;
use axum::{extract::State, response::Html};
use std::sync::Arc;

use crate::{
    db::{connector::DbCon, responses::AppDbResponse, stats::GameStats},
    handlers::errors::InternalErrorTemplate,
};

#[derive(Template)]
#[template(path = "stats_table.html")]
pub struct GameStatsTable {
    game_stats: GameStats,
}

pub async fn game_stats_table(
    State(db_con): State<Arc<DbCon>>,
) -> Result<Html<String>, Html<String>> {
    match db_con.get_game_stats().await {
        AppDbResponse::GameStats(game_stats) => Ok(Html(GameStatsTable { game_stats }.to_string())),
        _ => Err(Html(InternalErrorTemplate {}.to_string())),
    }
}

#[derive(Template)]
#[template(path = "stats_chart.html")]
pub struct GameStatsChart {
    game_stats: GameStats,
}

pub async fn game_stats_chart(
    State(db_con): State<Arc<DbCon>>,
) -> Result<Html<String>, Html<String>> {
    match db_con.get_game_stats().await {
        AppDbResponse::GameStats(game_stats) => Ok(Html(GameStatsChart { game_stats }.to_string())),
        _ => Err(Html(InternalErrorTemplate {}.to_string())),
    }
}
