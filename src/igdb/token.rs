use anyhow::Context;
use serde::Deserialize;
use serde_json;
use std::time::Instant;

// Used to deserialize the igdb token request response
#[derive(Debug, Clone, Deserialize)]
pub struct TokenRequestResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

#[derive(Debug, Clone)]
pub struct IgdbToken {
    pub value: String,
    pub expire_in: u64,
    pub acquired_at: Instant,
}

impl IgdbToken {
    pub async fn get(client_id: &str, client_secret: &str) -> anyhow::Result<Self> {
        let igdb_token_request_url = format!(
    "https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials",
    client_id, client_secret);
        let client = reqwest::Client::new();
        let token_request = client
            .post(igdb_token_request_url)
            .body("")
            .send()
            .await
            .context("Failed to receive token request response.")?
            .text()
            .await
            .context("Failed to convert token request response to string.")?;
        let token_response: TokenRequestResponse = serde_json::from_str(&token_request)
            .context("Failed to deserialize token request response.")?;
        let token = Self::from_token_response(token_response);
        Ok(token)
    }
    pub fn from_token_response(token_response: TokenRequestResponse) -> Self {
        IgdbToken {
            value: token_response.access_token,
            expire_in: token_response.expires_in,
            acquired_at: Instant::now(),
        }
    }
    pub fn has_expired(&self) -> bool {
        self.acquired_at.elapsed().as_secs() > self.expire_in
    }
}
