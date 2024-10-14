use std::collections::HashMap;

use serde::Deserialize;

use crate::db::data::{GameMetaData, Metacritic};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct SteamMetacritic {
    pub score: usize,
    pub url: String,
}

impl From<SteamMetacritic> for Metacritic {
    fn from(value: SteamMetacritic) -> Self {
        Self {
            score: value.score,
            url: value.url,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Screenshot {
    pub path_thumbnail: String,
}

#[derive(Deserialize, Default)]
pub struct SteamMetaData {
    pub about_the_game: String,
    pub short_description: String,
    pub screenshots: Vec<Screenshot>,
    pub metacritic: Option<SteamMetacritic>,
    #[serde(skip_deserializing)]
    pub cover: String,
}

impl From<SteamMetaData> for GameMetaData {
    fn from(value: SteamMetaData) -> Self {
        Self {
            summary: Some(value.short_description),
            story_line: None,
            screenshots: value
                .screenshots
                .into_iter()
                .map(|x| x.path_thumbnail)
                .collect(),
            cover: Some(value.cover),
            metacritic: value.metacritic.map(|x| x.into()),
        }
    }
}

#[derive(Deserialize)]
struct SteamData {
    pub success: bool,
    #[serde(rename(deserialize = "data"))]
    pub details: SteamMetaData,
}

#[derive(Debug)]
pub struct SteamClient {
    url: &'static str,
}

impl Default for SteamClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SteamClient {
    pub fn new() -> Self {
        Self {
            url: "https://store.steampowered.com/api/appdetails?appids=",
        }
    }
    pub async fn fetch_metadata(&self, steam_id: usize) -> anyhow::Result<SteamMetaData> {
        let cover = format!(
            "https://shared.steamstatic.com/store_item_assets/steam/apps/{}/hero_capsule.jpg",
            steam_id
        );
        let re: String = reqwest::get(format!("{}{}", self.url, steam_id))
            .await?
            .text()
            .await?;

        let mut re: HashMap<usize, SteamData> = serde_json::from_str(&re)?;
        let mut metadata: SteamMetaData = match re.remove(&steam_id) {
            Some(data) => {
                let SteamData {
                    success: __s,
                    details: data,
                } = data;
                data
            }
            None => SteamMetaData::default(),
        };
        // better safe than sorry
        metadata.about_the_game = ammonia::clean(&metadata.about_the_game);
        metadata.screenshots.truncate(4);
        metadata.cover = cover;
        Ok(metadata)
    }
}
