pub mod connector;
pub mod data;
pub mod requests;
pub mod responses;
pub mod stats;

use chrono::{Duration, Local};
use libpobsd::{Game, GameDataBase, SearchType};
use rand::prelude::*;
use std::collections::{HashMap, HashSet};
use tokio::sync::{
    mpsc::{self, Sender as MpscSender},
    oneshot::{self},
};

use crate::{
    app::{pagination::Paginator, representation::GameRepresentation},
    db::{
        connector::DbCon, data::GameMetaData, requests::AppDbRequest, responses::AppDbResponse,
        stats::GameStats,
    },
    fetcher::FetcherMessage,
    utils::GameFilterWrapper,
};

pub struct AppDb {
    stats: GameStats,
    last_updated: String,
    games: GameDataBase,
    game_ids: HashSet<u32>,
    game_metadata: HashMap<u32, GameMetaData>,
    igdb_tx: MpscSender<FetcherMessage>,
}

impl AppDb {
    pub fn new(games: GameDataBase, igdb_tx: MpscSender<FetcherMessage>) -> Self {
        let game_ids: HashSet<u32> = games.get_all_games().into_iter().map(|g| g.uid).collect();
        Self {
            games,
            game_ids,
            last_updated: chrono::Utc::now().to_rfc2822(),
            game_metadata: HashMap::default(),
            igdb_tx,
            stats: GameStats::default(),
        }
    }
    pub fn game_has_metadata(&self, game_id: u32) -> bool {
        match self.games.get_game_by_id(game_id) {
            Some(game) => game.igdb_id.is_some() || game.get_steam_id().is_some(),
            None => false,
        }
    }
    pub fn get_game_by_id(&self, game_id: u32) -> AppDbResponse {
        match self.games.get_game_by_id(game_id) {
            Some(game) => AppDbResponse::Game(game.clone()),
            None => AppDbResponse::NoGame,
        }
    }
    pub fn get_game_stats(&self) -> AppDbResponse {
        AppDbResponse::GameStats(self.stats.clone())
    }
    pub fn get_all_games(&self) -> AppDbResponse {
        let games: Vec<Game> = self.games.get_all_games().into_iter().cloned().collect();
        AppDbResponse::AllGames(games)
    }
    pub async fn get_game_representation_by_id(&self, game_id: u32) -> AppDbResponse {
        match self.get_game_by_id(game_id) {
            AppDbResponse::Game(game) => {
                if self.game_has_metadata(game_id) {
                    match self.game_metadata.get(&game_id) {
                        Some(metadata) => {
                            let gr = GameRepresentation {
                                game: game.clone(),
                                metadata: Some(metadata.clone()),
                            };
                            AppDbResponse::GameRepresentation(gr)
                        }
                        None => {
                            let (tx, rx) = oneshot::channel::<GameMetaData>();
                            let _ = self
                                .igdb_tx
                                .send(FetcherMessage {
                                    igdb_id: game.igdb_id,
                                    steam_id: game.get_steam_id(),
                                    game_name: game.name.clone(),
                                    sender: tx,
                                })
                                .await;
                            AppDbResponse::Pending(rx)
                        }
                    }
                } else {
                    let gr = GameRepresentation {
                        game: game.clone(),
                        metadata: None,
                    };
                    AppDbResponse::GameRepresentation(gr)
                }
            }
            AppDbResponse::NoGame => AppDbResponse::NoGame,
            _ => unreachable!("Other variants cannot be returned by get_game_by_id"),
        }
    }
    pub fn get_game_list(&self, game_ids: Vec<u32>) -> AppDbResponse {
        let mut games: Vec<Game> = vec![];
        for game_id in game_ids {
            if let Some(game) = self.games.get_game_by_id(game_id) {
                games.push(game.clone());
            }
        }
        AppDbResponse::GameList(games)
    }
    pub fn get_game_list_paginated_filtered(
        &self,
        page: usize,
        filter: GameFilterWrapper,
    ) -> AppDbResponse {
        let games: Vec<Game> = if filter.is_empty() {
            self.games.get_all_games().into_iter().cloned().collect()
        } else {
            self.games
                .search_game_by_filter(&SearchType::NotCaseSensitive, &filter)
                .into_iter()
                .cloned()
                .collect()
        };
        match Paginator::new(games.len(), 15).page(page) {
            Some(page) => AppDbResponse::GameListPaginated(
                games[page.first_element..=page.last_element].to_vec(),
                page,
            ),
            // It can be None because of an error or the list of games
            // is empty and the Paginator returns None
            None => {
                if games.is_empty() {
                    AppDbResponse::NoGame
                } else {
                    AppDbResponse::Error
                }
            }
        }
    }
    pub fn get_recent_games(&self) -> AppDbResponse {
        let now = Local::now().naive_local().date();
        let mut games: Vec<&Game> = self
            .games
            .get_all_games()
            .into_iter()
            .filter(|a| now - a.added < Duration::try_days(30).unwrap())
            .collect();
        games.sort_by(|a, b| a.added.partial_cmp(&b.added).unwrap());
        games.reverse();
        let mut grs: Vec<GameRepresentation> = vec![];
        for game in games {
            grs.push(GameRepresentation {
                game: game.clone(),
                metadata: self.game_metadata.get(&game.uid).cloned(),
            })
        }
        AppDbResponse::RecentGames(grs)
    }
    pub fn launch(mut self) -> DbCon {
        self.update_stats();
        let (tx_read, mut rx_read) = mpsc::channel::<AppDbRequest>(150);
        let (tx_write, mut rx_write) = mpsc::channel::<AppDbRequest>(150);
        tokio::spawn(async move {
            loop {
                let adbr: AppDbRequest = tokio::select! {
                    Some(abr) = rx_write.recv() => {
                        abr
                    }
                    Some(abr) = rx_read.recv() => {
                       abr
                    }
                };
                match adbr {
                    AppDbRequest::GetGame { game_id, ack } => {
                        let _ = ack.send(self.get_game_by_id(game_id));
                    }
                    AppDbRequest::GetGameIds { ack } => {
                        let game_ids = self.game_ids.clone();
                        let _ = ack.send(AppDbResponse::GameIds(game_ids));
                    }
                    AppDbRequest::GetGameStats { ack } => {
                        let _ = ack.send(self.get_game_stats());
                    }
                    AppDbRequest::GetAllGames { ack } => {
                        let _ = ack.send(self.get_all_games());
                    }
                    AppDbRequest::GetGameList { game_ids, ack } => {
                        let _ = ack.send(self.get_game_list(game_ids));
                    }
                    AppDbRequest::GetGameListPaginatedFiltered { page, filter, ack } => {
                        let _ = ack.send(self.get_game_list_paginated_filtered(page, filter));
                    }
                    AppDbRequest::GetGameRepresentation { game_id, ack } => {
                        let _ = ack.send(self.get_game_representation_by_id(game_id).await);
                    }
                    AppDbRequest::GetRandomUid { ack } => {
                        let games = self.games.get_all_games();
                        let game_number = games.count;
                        let rid = thread_rng().gen_range(0..game_number);
                        let _ = ack.send(AppDbResponse::RandomUid(games.into_inner()[rid].uid));
                    }
                    AppDbRequest::GetRecentGames { ack } => {
                        let _ = ack.send(self.get_recent_games());
                    }
                    AppDbRequest::InsertMetadata {
                        game_id,
                        metadata,
                        ack,
                    } => {
                        self.game_metadata.insert(game_id, metadata);
                        let _ = ack.send(AppDbResponse::Done);
                    }
                    AppDbRequest::UpdateDb { game_db, ack } => {
                        self.games = game_db;
                        self.update_stats();
                        let _ = ack.send(AppDbResponse::Done);
                        self.last_updated = chrono::Utc::now().to_rfc2822();
                    }
                }
            }
        });
        DbCon { tx_read, tx_write }
    }
}
