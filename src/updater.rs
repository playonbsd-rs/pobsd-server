use libpobsd::Game;
use std::{collections::HashSet, sync::Arc};
use tokio::sync::{mpsc::Sender as MpscSender, oneshot};

use crate::{
    db::{connector::DbCon, data::GameMetaData, responses::AppDbResponse},
    fetcher::FetcherMessage,
    utils::{get_db, DB_URL},
};

pub struct GameDbUpdater {
    db_con: Arc<DbCon>,
    lptx: MpscSender<FetcherMessage>,
}

impl GameDbUpdater {
    pub fn init(db_con: Arc<DbCon>, lptx: MpscSender<FetcherMessage>) -> Self {
        Self { db_con, lptx }
    }
    pub fn launch(&self) {
        tokio::spawn({
            let db_con = self.db_con.clone();
            let lptx = self.lptx.clone();
            async move {
                tracing::debug!("Launching game database updater");
                let game_ids = match db_con.get_game_ids().await {
                    AppDbResponse::GameIds(game_ids) => game_ids,
                    _ => unreachable!("GameIds is the only possible variant"),
                };
                let to_fetch = match db_con.get_all_games().await {
                    AppDbResponse::AllGames(games) => get_game_ids(games),
                    _ => vec![],
                };
                background_fetching(db_con.clone(), lptx.clone(), to_fetch);
                tokio::time::sleep(tokio::time::Duration::from_secs(500)).await;
                loop {
                    match get_db(DB_URL).await.ok() {
                        Some(db) => {
                            let new_game_ids: HashSet<u32> =
                                db.get_all_games().into_iter().map(|g| g.uid).collect();
                            if let AppDbResponse::Done = db_con.update_db(db).await {
                                tracing::debug!("The game database has been updated.");
                                tracing::debug!("Fetching metadata for new games.");
                                // return the game_id for game that are present in the new
                                // database but not the old one
                                let diff: Vec<u32> =
                                    new_game_ids.difference(&game_ids).copied().collect();
                                background_fetching(db_con.clone(), lptx.clone(), diff);
                            } else {
                                tracing::debug!(
                                "Failed to update the game database. Trying again in 500 seconds"
                            );
                            }
                        }
                        None => tracing::debug!(
                            "Failed to update the game database. Trying again in 500 seconds"
                        ),
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(500)).await;
                }
            }
        });
    }
}

pub fn background_fetching(
    db_con: Arc<DbCon>,
    lptx: MpscSender<FetcherMessage>,
    to_fetch: Vec<u32>,
) {
    tokio::spawn({
        async move {
            for game_id in to_fetch {
                tokio::time::sleep(tokio::time::Duration::from_millis(350)).await;
                if let AppDbResponse::Game(game) = db_con.get_game(game_id).await {
                    let (tx, rx) = oneshot::channel::<GameMetaData>();
                    let msg = FetcherMessage {
                        igdb_id: game.igdb_id,
                        game_name: game.name.clone(),
                        sender: tx,
                        steam_id: game.get_steam_id(),
                    };
                    match lptx.send(msg).await {
                        Ok(_) => {
                            if let Ok(metadata) = rx.await {
                                db_con.insert_metadata(game_id, metadata).await;
                                tracing::debug!("Background fetch for {}", game_id)
                            }
                        }
                        Err(_) => {
                            tracing::debug!("Error while doing background fetch for {:?}", game_id)
                        }
                    }
                }
            }
        }
    });
}

pub fn get_game_ids(mut games: Vec<Game>) -> Vec<u32> {
    games.sort_by(|a, b| a.added.cmp(&b.added));
    let mut game_ids: Vec<u32> = games.into_iter().map(|x| x.uid).collect();
    game_ids.reverse();
    game_ids
}
