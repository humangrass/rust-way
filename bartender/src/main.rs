use crate::app::{AppState, JWTState};
use crate::cli::Cli;
use log::{info, warn};
use multitool_hg::database::postgres::new_postgres_pool;
use multitool_hg::logger::tracer_logger::new_tracer_logger;
use std::path::Path;
use std::process;
use std::sync::Arc;
use tokio::signal;

mod api;
mod app;
mod cli;
mod config;
mod entities;
mod repository;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Fatal error occurred: {}", err);
        process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let cli = Cli::new();
    new_tracer_logger(cli.log_level);

    let config =
        config::BartenderConfig::new(Path::new(&cli.config)).expect("Failed to load config");

    let database_pool = new_postgres_pool(config.database)
        .await
        .expect("Failed to create Postgres pool");
    let app_state = Arc::new(AppState::new(
        database_pool,
        JWTState {
            jwt_secret: config.app.jwt_secret,
            access_token_expiration: config.app.access_token_expiration,
            refresh_token_expiration: config.app.refresh_token_expiration,
        },
    ));

    let app = api::create_router(app_state);
    let address = format!("{}:{}", config.app.host, config.app.port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("Failed to bind");
    let server = async {
        axum::serve(listener, app)
            .await
            .expect("Failed to run server");
    };

    info!("Server started on http://{}", address);
    info!("Docs available here http://{}/docs", address);

    let shutdown_signal = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
        warn!("Receive stop signal. Start shutdown process...");
    };

    tokio::select! {
        _ = server => {
            warn!("The server has terminated its work.");
        }
        _ = shutdown_signal => {
            warn!("Graceful shutdown initiated...");
        }
    }

    Ok(())
}
