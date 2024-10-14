use anyhow::Context;
use libpobsd::{GameDataBase, GameFilter, Parser, ParserResult};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use tracing::{level_filters::LevelFilter, Level, Subscriber};
use tracing_subscriber::{filter::Targets, fmt, prelude::*, EnvFilter};

pub const DB_URL: &str =
    "https://raw.githubusercontent.com/playonbsd/OpenBSD-Games-Database/main/openbsd-games.db";
pub const UPDATE_PERIOD: u64 = 500;

pub async fn get_db(db_url: &str) -> anyhow::Result<GameDataBase> {
    let req = reqwest::get(db_url)
        .await
        .context("Failed to fetch playonbsd database")?;
    let content = req
        .text()
        .await
        .context("Failed to read playonbsd database content")?;
    let games = match Parser::default().load_from_string(&content) {
        ParserResult::WithoutError(games) => games,
        ParserResult::WithError(games, _) => games,
    };
    let db = GameDataBase::new(games);
    Ok(db)
}

fn add_query_string(field: &Option<String>, field_name: &str, query_string: &mut Vec<String>) {
    if let Some(field_value) = field {
        query_string.push(format!("{}={}", field_name, field_value))
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(transparent)]
pub struct GameFilterWrapper(GameFilter);

impl GameFilterWrapper {
    pub fn query_string(&self) -> String {
        let mut re: Vec<String> = vec![];
        add_query_string(&self.name, "name", &mut re);
        add_query_string(&self.engine, "engine", &mut re);
        add_query_string(&self.runtime, "runtime", &mut re);
        add_query_string(&self.genre, "genre", &mut re);
        add_query_string(&self.tag, "tag", &mut re);
        add_query_string(&self.year, "year", &mut re);
        add_query_string(&self.dev, "dev", &mut re);
        add_query_string(&self.publi, "publi", &mut re);
        if let Some(ref x) = self.status {
            re.push(format!("status={}", x))
        }
        re.join("&")
    }
    pub fn new(filter: GameFilter) -> Self {
        GameFilterWrapper(filter)
    }

    pub fn from_pattern(pattern: &str) -> Self {
        let mut filter = GameFilter::default();
        filter.set_name(pattern);
        filter.set_engine(pattern);
        filter.set_runtime(pattern);
        filter.set_genre(pattern);
        filter.set_tag(pattern);
        filter.set_year(pattern);
        filter.set_dev(pattern);
        filter.set_publi(pattern);
        Self(filter)
    }
}

impl Deref for GameFilterWrapper {
    type Target = GameFilter;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GameFilterWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn get_subscriber() -> impl Subscriber {
    let target_filter = Targets::new().with_target("pobsd_server", Level::TRACE);
    let env_filter = EnvFilter::builder()
        .with_env_var("POBSD_SERVER_LOG")
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(target_filter)
        .with(env_filter)
}
