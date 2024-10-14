use libpobsd::GameDataBase;
use tokio::sync::oneshot::Sender as OsSender;

use crate::{
    db::{data::GameMetaData, responses::AppDbResponse},
    utils::GameFilterWrapper,
};

pub enum AppDbRequest {
    GetGame {
        game_id: u32,
        ack: OsSender<AppDbResponse>,
    },
    GetGameIds {
        ack: OsSender<AppDbResponse>,
    },
    GetAllGames {
        ack: OsSender<AppDbResponse>,
    },
    GetGameStats {
        ack: OsSender<AppDbResponse>,
    },
    GetGameList {
        game_ids: Vec<u32>,
        ack: OsSender<AppDbResponse>,
    },
    GetGameListPaginatedFiltered {
        page: usize,
        filter: GameFilterWrapper,
        ack: OsSender<AppDbResponse>,
    },
    GetGameRepresentation {
        game_id: u32,
        ack: OsSender<AppDbResponse>,
    },
    GetRandomUid {
        ack: OsSender<AppDbResponse>,
    },
    GetRecentGames {
        ack: OsSender<AppDbResponse>,
    },
    InsertMetadata {
        game_id: u32,
        metadata: GameMetaData,
        ack: OsSender<AppDbResponse>,
    },
    UpdateDb {
        game_db: GameDataBase,
        ack: OsSender<AppDbResponse>,
    },
}
