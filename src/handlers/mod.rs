pub mod about;
pub mod assets;
pub mod errors;
pub mod game;
pub mod list;
pub mod rss;
pub mod stats;

use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::db::connector::DbCon;

pub fn get_router(db_con: Arc<DbCon>) -> Router {
    Router::new()
        .route("/", get(list::game_list).post(list::game_list_search))
        .route("/static/bulma.min.css", get(assets::get_bulma_css))
        .route("/favicon.ico", get(assets::get_favicon))
        .route("/static/fontawesome5.min.css", get(assets::get_awesome_css))
        .route("/static/charts.min.css", get(assets::get_charts_css))
        .route(
            "/webfonts/fa-solid-900.woff2",
            get(assets::get_fa_solid_900),
        )
        .route("/random", get(game::game_details_random))
        .route("/rss", get(rss::rss_feed))
        .route("/news", get(list::news))
        .route("/:game_id", get(game::game_details))
        .route("/stats_table", get(stats::game_stats_table))
        .route("/stats_chart", get(stats::game_stats_chart))
        .route("/about", get(about::about_page()))
        .with_state(db_con)
        .layer(TraceLayer::new_for_http())
}
