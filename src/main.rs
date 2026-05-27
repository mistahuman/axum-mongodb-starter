use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use mongodb::Client;
use serde_json::{Value, json};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod handlers;
mod models;
mod schemas;

pub struct AppState {
    pub db: mongodb::Database,
    pub app_name: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cfg = config::Config::load();

    let client = Client::with_uri_str(&cfg.mongo_uri)
        .await
        .expect("failed to connect to MongoDB");
    client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1})
        .await
        .expect("failed to ping MongoDB");
    info!("connected to MongoDB, db={}", cfg.mongo_db);

    let state = Arc::new(AppState {
        db: client.database(&cfg.mongo_db),
        app_name: cfg.app_name.clone(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .merge(handlers::exampleitem::router())
        .layer(cors)
        .with_state(state);

    let addr = format!("0.0.0.0:{}", cfg.port);
    info!(
        "server starting on {}, app={}, version={}",
        addr, cfg.app_name, cfg.app_version
    );
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(state): State<Arc<AppState>>) -> Json<Value> {
    Json(json!({"msg": format!("Hello {}!", state.app_name)}))
}
