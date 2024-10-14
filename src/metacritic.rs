use anyhow::anyhow;
use scraper::Selector;
use serde::Deserialize;

use crate::db::data::{GameMetaData, Metacritic};

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    pub rating_value: usize,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetacriticMetaData {
    pub name: String,
    pub description: String,
    pub aggregate_rating: Option<Rating>,
    pub url: String,
}

impl From<MetacriticMetaData> for GameMetaData {
    fn from(value: MetacriticMetaData) -> Self {
        let metacritic = value.aggregate_rating.map(|x| Metacritic {
            score: x.rating_value,
            url: value.url,
        });
        Self {
            summary: Some(value.description),
            story_line: None,
            cover: None,
            metacritic,
            screenshots: vec![],
        }
    }
}

#[derive(Debug)]
pub struct MetacriticClient {
    url: &'static str,
}

impl Default for MetacriticClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MetacriticClient {
    pub fn new() -> Self {
        Self {
            url: "https://www.metacritic.com/game/",
        }
    }
    pub async fn fetch_metadata(&self, game_name: &str) -> anyhow::Result<MetacriticMetaData> {
        let game_name = game_name.to_string().replace("\'", "");
        let url: String = format!("{}{}", self.url, slug::slugify(game_name));
        tracing::debug!("fetching metacritic game: {}", &url);
        let re: String = reqwest::get(&url).await?.text().await?;
        let document = scraper::Html::parse_document(&re);
        let selector = "script[type=\"application/ld+json\"]";
        match Selector::parse(selector) {
            Ok(selector) => {
                if let Some(element) = document.select(&selector).next() {
                    if let Some(text) = element.text().next() {
                        let data: MetacriticMetaData = serde_json::from_str(text)?;
                        return Ok(data);
                    }
                }
                Err(anyhow!("Failed to find metadata"))
            }
            Err(_) => Err(anyhow!("Failed to find metadata")),
        }
    }
}
