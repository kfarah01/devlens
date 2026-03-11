use axum::{routing::get, Router, Json, extract::State};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::sync::Arc;

struct AppState {
    db: PgPool,
    python_url: String,
}

async fn health(State(state): State<Arc<AppState>>) -> Json<Value> {
    let db_ok = sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .is_ok();

    let python_ok = reqwest::get(format!("{}/health", state.python_url))
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false);

    Json(json!({
        "status": "ok",
        "service": "devlens-api",
        "db": db_ok,
        "python_service": python_ok
    }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let python_url = std::env::var("PYTHON_SERVICE_URL")?;

    let db = PgPool::connect(&database_url).await?;
    let state = Arc::new(AppState { db, python_url });

    let app = Router::new()
        .route("/health", get(health))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    tracing::info!("Rust API listening on 0.0.0.0:3001");
    axum::serve(listener, app).await?;

    Ok(())
}
