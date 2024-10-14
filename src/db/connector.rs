use libpobsd::GameDataBase;
use tokio::sync::{
    mpsc::Sender as MpscSender,
    oneshot::{self, Receiver as OsReceiver},
};

use crate::{
    db::{data::GameMetaData, requests::AppDbRequest, responses::AppDbResponse},
    utils::GameFilterWrapper,
};

#[derive(Debug, Clone)]
pub struct DbCon {
    pub tx_read: MpscSender<AppDbRequest>,
    pub tx_write: MpscSender<AppDbRequest>,
}

impl DbCon {
    pub fn new(tx_read: MpscSender<AppDbRequest>, tx_write: MpscSender<AppDbRequest>) -> Self {
        Self { tx_read, tx_write }
    }
    pub async fn get_game(&self, game_id: u32) -> AppDbResponse {
        let (ack, rx) = oneshot::channel::<AppDbResponse>();
        self.send(AppDbRequest::GetGame { game_id, ack }, rx).await
    }
    pub async fn get_game_stats(&self) -> AppDbResponse {
        let (ack, rx) = oneshot::channel::<AppDbResponse>();
        self.send(AppDbRequest::GetGameStats { ack }, rx).await
    }
    pub async fn get_all_games(&self) -> AppDbResponse {
        let (ack, rx) = oneshot::channel::<AppDbResponse>();
        self.send(AppDbRequest::GetAllGames { ack }, rx).await
    }
    pub async fn get_game_representation(&self, game_id: u32) -> AppDbResponse {
        let (ack, rx) = oneshot::channel::<AppDbResponse>();
        self.send(AppDbRequest::GetGameRepresentation { game_id, ack }, rx)
            .await
    }
    pub async fn get_game_list(&self, game_ids: Vec<u32>) -> AppDbResponse {
        let (ack, rx) = oneshot::channel::<AppDbResponse>();
        self.send(AppDbRequest::GetGameList { game_ids, ack }, rx)
            .await
    }
    pub async fn insert_metadata(&self, game_id: u32, metadata: GameMetaData) -> AppDbResponse {
        let (ack, rx) = oneshot::channel::<AppDbResponse>();
        self.send(
            AppDbRequest::InsertMetadata {
                game_id,
                metadata,
                ack,
            },
            rx,
        )
        .await
    }
    pub async fn get_game_list_paginated_filtered(
        &self,
        page: usize,
        filter: GameFilterWrapper,
    ) -> AppDbResponse {
        let (ack, rx) = oneshot::channel::<AppDbResponse>();
        self.send(
            AppDbRequest::GetGameListPaginatedFiltered { page, filter, ack },
            rx,
        )
        .await
    }
    pub async fn update_db(&self, game_db: GameDataBase) -> AppDbResponse {
        let (ack, rx) = oneshot::channel::<AppDbResponse>();
        self.send(AppDbRequest::UpdateDb { game_db, ack }, rx).await
    }
    pub async fn get_random_uid(&self) -> AppDbResponse {
        let (ack, rx) = oneshot::channel::<AppDbResponse>();
        self.send(AppDbRequest::GetRandomUid { ack }, rx).await
    }
    pub async fn get_recent_games(&self) -> AppDbResponse {
        let (ack, rx) = oneshot::channel::<AppDbResponse>();
        self.send(AppDbRequest::GetRecentGames { ack }, rx).await
    }
    pub async fn get_game_ids(&self) -> AppDbResponse {
        let (ack, rx) = oneshot::channel::<AppDbResponse>();
        self.send(AppDbRequest::GetGameIds { ack }, rx).await
    }
    pub async fn send(
        &self,
        app_db_request: AppDbRequest,
        rx: OsReceiver<AppDbResponse>,
    ) -> AppDbResponse {
        let tx = match app_db_request {
            AppDbRequest::InsertMetadata {
                game_id: _,
                metadata: _,
                ack: _,
            }
            | AppDbRequest::UpdateDb { game_db: _, ack: _ } => self.tx_write.clone(),
            _ => self.tx_read.clone(),
        };
        match tx.send(app_db_request).await {
            Ok(_) => match rx.await {
                Ok(app_db_response) => app_db_response,
                Err(_) => AppDbResponse::Error,
            },
            Err(_) => AppDbResponse::Error,
        }
    }
}
