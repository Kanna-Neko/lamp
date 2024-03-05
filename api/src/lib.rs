mod controllers;
mod data;
mod models;
mod services;
mod error;

use config::{File,Config};
use tokio::signal;
use std::sync::Arc;
use clap::Parser;
use controllers::AdminUseCase;
use data::{ProjectRepo,TransactionClient, ScriptFolderRepo, ScriptRepo};
use serde::Deserialize;
use services::{AdminService, Services};
#[derive(Deserialize,Debug)]
pub struct AppConfig {
    bind_ip:String,
    port:i32,
    mongodb_uri:String
}

#[derive(Parser,Debug)]
pub struct Cli {
    #[arg(short, long, default_value_t = String::from("configs/dev.yaml"))]
    pub config: String
}

pub fn read_config(path:&str) -> AppConfig {
    let config:AppConfig = Config::builder()
        .add_source(File::with_name(path))
        .build().expect("read config error")
        .try_deserialize()
        .expect("deserialize error");
    config
}

pub async fn start(config: AppConfig) {
    // init data client
    let mongo_client = Arc::new(mongodb::Client::with_uri_str(config.mongodb_uri).await.expect("connect mongodb error"));
    let transaction_client = Arc::new(TransactionClient::new(Arc::clone(&mongo_client)));

    // init repo
    let project_repo = Arc::new(ProjectRepo::new(Arc::clone(&mongo_client)));
    let script_folder_repo = Arc::new(ScriptFolderRepo::new(Arc::clone(&mongo_client)));
    let script_repo = Arc::new(ScriptRepo::new(Arc::clone(&mongo_client)));

    // init use case
    let admin_use_case = Arc::new(AdminUseCase::new(transaction_client.clone(),project_repo.clone(),script_folder_repo.clone(),script_repo.clone()));

    // init service
    let admin_service = AdminService::new(admin_use_case);
    let service = Services::new(admin_service);
    let app = service.router();

    // run our app with hyper, listening globally.
    let listener = tokio::net::TcpListener::bind(format!("{}:{}",config.bind_ip,config.port)).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}