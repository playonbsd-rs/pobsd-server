use tokio::{
    sync::{
        mpsc::{self, Sender as MspcSender},
        oneshot::Sender as OneShotSender,
    },
    task::JoinHandle,
};

use crate::{
    db::data::GameMetaData,
    igdb::client::{IgdbClient, IgdbMetaData},
    metacritic::MetacriticClient,
    steam::{SteamClient, SteamMetaData},
};

#[derive(Debug)]
pub struct FetcherMessage {
    pub igdb_id: Option<usize>,
    pub steam_id: Option<usize>,
    pub game_name: String,
    pub sender: OneShotSender<GameMetaData>,
}

pub struct MetaDataFetcher {
    // used to receive background fetching
    pub low_priority: MspcSender<FetcherMessage>,
    // used to receive request related fetching
    pub high_priority: MspcSender<FetcherMessage>,
    pub join_handler: tokio::task::JoinHandle<anyhow::Result<()>>,
}

impl MetaDataFetcher {
    pub async fn init(client_id: String, client_secret: String) -> Self {
        // used to prefetch medatada and put them in cache
        // it is low priority vs metadata used in response
        // to requests
        let (lptx, mut lprx) = mpsc::channel(300);
        // used to provide metadata in response to requests
        // is high priority (results are cached)
        let (hptx, mut hprx) = mpsc::channel(300);
        // obtain the Client before spawning the thread so we are sure the client is
        // ready when the metadata requests arrive. Need to be mutable to be able to
        // refresh the token when fetching.
        let mut igdb_client = IgdbClient::new(client_id.clone(), client_secret.clone())
            .await
            .expect("Fail to obtain a Igdb client. Aborting");
        let steam_client = SteamClient::new();
        let metacritic_client = MetacriticClient::new();
        let join_handler: JoinHandle<anyhow::Result<()>> = tokio::spawn({
            async move {
                loop {
                    let fetcher_message: FetcherMessage = tokio::select! {
                        // using biased select, hprx takes precedente over lprx
                        biased;
                        Some(h) = hprx.recv() => {
                            h
                        }
                        Some(l) = lprx.recv() => {
                            l
                        }
                    };
                    let igdb_metadata = match fetcher_message.igdb_id {
                        Some(igdb_id) => match igdb_client.fetch_metadata(igdb_id).await {
                            Ok(game_metadata) => game_metadata,
                            Err(e) => {
                                tracing::debug!("Error while fetching igdb metadata: {e}");
                                IgdbMetaData::default()
                            }
                        },
                        None => IgdbMetaData::default(),
                    };
                    let steam_metadata = match fetcher_message.steam_id {
                        Some(steam_id) => match steam_client.fetch_metadata(steam_id).await {
                            Ok(game_metadata) => game_metadata,
                            Err(e) => {
                                tracing::debug!("Error while fetching steam metadata: {e}");
                                SteamMetaData::default()
                            }
                        },
                        None => SteamMetaData::default(),
                    };
                    let metacritic_metadata = metacritic_client
                        .fetch_metadata(&fetcher_message.game_name)
                        .await
                        .unwrap_or_default();

                    let metadata: GameMetaData = igdb_metadata.into();
                    let metadata = metadata
                        .merge(metacritic_metadata.into())
                        .merge(steam_metadata.into());

                    if let Err(e) = fetcher_message.sender.send(metadata) {
                        tracing::debug!("Could not send back result: {:?}", e);
                    };
                }
            }
        });
        Self {
            join_handler,
            low_priority: lptx,
            high_priority: hptx,
        }
    }
}
