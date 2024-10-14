use anyhow::anyhow;
use igdb::client::{CoversClient, GamesClient, IGDBClient, ScreenshotsClient};

use crate::{db::data::GameMetaData, igdb::token::IgdbToken};

#[derive(Debug, Clone, Default)]
pub struct IgdbMetaData {
    pub summary: String,
    pub story_line: String,
    pub screenshots: Vec<String>,
    pub cover: String,
}

impl From<IgdbMetaData> for GameMetaData {
    fn from(value: IgdbMetaData) -> Self {
        Self {
            summary: Some(value.summary),
            story_line: Some(value.story_line),
            screenshots: value.screenshots,
            cover: Some(value.cover),
            metacritic: None,
        }
    }
}

pub struct IgdbClient {
    pub client_id: String,
    pub client_secret: String,
    pub token: IgdbToken,
    pub games_client: GamesClient,
    pub screenshots_client: ScreenshotsClient,
    pub covers_client: CoversClient,
}

impl IgdbClient {
    pub async fn new(client_id: String, client_secret: String) -> anyhow::Result<Self> {
        let token = IgdbToken::get(&client_id, &client_secret).await?;
        let games_client = IGDBClient::new(&client_id, &token.value).games();
        let screenshots_client = IGDBClient::new(&client_id, &token.value).screenshots();
        let covers_client = IGDBClient::new(&client_id, &token.value).covers();
        Ok(Self {
            client_id,
            client_secret,
            token,
            games_client,
            screenshots_client,
            covers_client,
        })
    }

    pub async fn fetch_metadata(&mut self, igdb_id: usize) -> anyhow::Result<IgdbMetaData> {
        self.refresh_token().await?;
        tracing::debug!("fetching: {:?}", igdb_id);
        match self.games_client.get_first_by_id(igdb_id).await.ok() {
            Some(game) => {
                let screenshots = self
                    .screenshots_client
                    .get_by_game_id(game.id, 4)
                    .await
                    .unwrap_or_default()
                    .into_iter()
                    .map(|x| {
                        format!(
                            "https://images.igdb.com/igdb/image/upload/t_original/{}.jpg",
                            x.image_id
                        )
                    })
                    .collect();
                let cover = match self.covers_client.get_first_by_id(game.cover).await.ok() {
                    Some(cover) => format!(
                        "https://images.igdb.com/igdb/image/upload/t_cover_big/{}.png",
                        cover.image_id
                    ),
                    None => "".to_string(),
                };
                let data = IgdbMetaData {
                    summary: game.summary,
                    story_line: game.storyline,
                    screenshots,
                    cover,
                };
                Ok(data)
            }
            None => Err(anyhow!(
                "No metadata associated with the igdb_id {}",
                igdb_id
            )),
        }
    }

    pub async fn refresh_token(&mut self) -> anyhow::Result<()> {
        if self.token.has_expired() {
            let token = IgdbToken::get(&self.client_id, &self.client_secret).await?;
            let games_client = IGDBClient::new(&self.client_id, &self.token.value).games();
            let screenshots_client =
                IGDBClient::new(&self.client_id, &self.token.value).screenshots();
            let covers_client = IGDBClient::new(&self.client_id, &self.token.value).covers();
            self.token = token;
            self.games_client = games_client;
            self.screenshots_client = screenshots_client;
            self.covers_client = covers_client;
        }
        Ok(())
    }
}
