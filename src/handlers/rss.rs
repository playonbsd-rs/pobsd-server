use axum::{
    extract::State,
    http::{header, HeaderMap},
    response::Html,
};
use libpobsd::Game;
use rss::{ChannelBuilder, Item};
use std::sync::Arc;

use crate::{
    db::{connector::DbCon, responses::AppDbResponse},
    handlers::errors::InternalErrorTemplate,
};

pub async fn rss_feed(
    State(db_con): State<Arc<DbCon>>,
) -> Result<(HeaderMap, String), Html<String>> {
    match db_con.get_recent_games().await {
        AppDbResponse::RecentGames(recent_games) => {
            let games: Vec<Game> = recent_games.into_iter().map(|x| x.game).collect();
            let mut items: Vec<Item> = Vec::new();
            for game in games {
                let mut item = Item::default();
                if game.added.eq(&game.updated) {
                    item.set_title(format!("The game {} has been added.", &game.name))
                } else {
                    item.set_title(format!("The game {} has been updated.", &game.name))
                }
                item.set_pub_date(format!("{}", &game.updated.format("%F")));
                item.set_link(format!("https://pobsd.chocolatines.org/{}", game.uid));
                items.push(item);
            }
            let channel = ChannelBuilder::default()
                .title("PlayOnBSD updates")
                .link("https://playonbsd.com")
                .description("Game database updates")
                .items(items)
                .build();
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                "application/rss+xml;charset=UTF-8".parse().unwrap(),
            );
            Ok((headers, channel.to_string()))
        }
        _ => Err(Html(InternalErrorTemplate {}.to_string())),
    }
}
