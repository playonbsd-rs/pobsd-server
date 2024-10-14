use libpobsd::Game;

use crate::db::data::GameMetaData;

#[derive(Debug, Clone)]
pub struct GameRepresentation {
    pub game: Game,
    pub metadata: Option<GameMetaData>,
}

impl GameRepresentation {
    pub fn new(game: Game, metadata: Option<GameMetaData>) -> Self {
        Self { game, metadata }
    }
}
