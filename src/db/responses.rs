use libpobsd::Game;
use std::collections::HashSet;
use tokio::sync::oneshot::Receiver as OsReceiver;

use crate::{
    app::{pagination::Page, representation::GameRepresentation},
    db::{data::GameMetaData, stats::GameStats},
};

#[derive(Debug)]
pub enum AppDbResponse {
    Done,
    Error,
    Game(Game),
    GameStats(GameStats),
    GameIds(HashSet<u32>),
    AllGames(Vec<Game>),
    GameList(Vec<Game>),
    GameListPaginated(Vec<Game>, Page),
    GameMetaData(GameMetaData),
    GameRepresentation(GameRepresentation),
    NoMetaData,
    NoGame,
    Pending(OsReceiver<GameMetaData>),
    RandomUid(u32),
    RecentGames(Vec<GameRepresentation>),
}
