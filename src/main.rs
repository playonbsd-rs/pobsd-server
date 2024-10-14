use anyhow::Context;
use pledge::pledge_promises;
use pobsd_server::{
    app::config::AppConfig, cmd, db::AppDb, fetcher::MetaDataFetcher, handlers::get_router,
    updater::GameDbUpdater, utils,
};
use std::sync::Arc;
use unveil::unveil;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pledge_promises![Stdio Inet Rpath Dns Unveil].unwrap();
    // Needed to fetch the database
    unveil("/etc/ssl/cert.pem", "r")
        .or_else(unveil::Error::ignore_platform)
        .unwrap();

    // Construct a subscriber that prints formatted traces to stdout
    // use that subscriber as global default
    tracing::subscriber::set_global_default(utils::get_subscriber())?;

    // Get command line arguments
    let cmd_args = cmd::get_args().get_matches();

    // Load config file
    let config_file_path = cmd_args.get_one::<String>("config").unwrap();
    unveil(config_file_path, "r")
        .or_else(unveil::Error::ignore_platform)
        .unwrap();
    let config = AppConfig::from_init_file(config_file_path)?;
    pledge_promises![Stdio Inet Rpath Dns].unwrap();

    // Load the game database from GitHub
    let game_database = utils::get_db(utils::DB_URL).await?;

    // Launch the medatata fetcher
    let igdb_config = config.igdb_config;
    tracing::info!("Launching fetcher");
    let fetcher = MetaDataFetcher::init(igdb_config.client_id, igdb_config.client_secret).await;

    // Launch the Db and get the connector back
    let db_con = Arc::new(AppDb::new(game_database, fetcher.high_priority.clone()).launch());

    // Launch the regular update of game data and metadata
    GameDbUpdater::init(db_con.clone(), fetcher.low_priority.clone()).launch();

    // Launch the router
    let router = get_router(db_con);

    // Start listening for request
    let listener = tokio::net::TcpListener::bind(config.server_config.to_string())
        .await
        .unwrap();

    tracing::info!("Listening to incoming requests");
    axum::serve(listener, router).await.unwrap();

    // Join the remaining task
    let _ = fetcher.join_handler.await.context("Fail to join tasks")?;
    tracing::info!("Shutting down fetcher");

    Ok(())
}
