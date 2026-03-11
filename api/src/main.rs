use axum::{routing::get, Router, Json, extract::State};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::{net::SocketAddr, sync::Arc};

struct AppState {
    db: PgPool,
    python_url: String,
}

async fn health(State(state): State<Arc<AppState>>) -> Json<Value> {
    // Test DB
    let db_ok = sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .is_ok();

    // Call Python health
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

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("Rust API listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
```

---

## Step 8 — Root files & push (Task 10)

Create `.gitignore` at the repo root:
```
# Rust
api/target/

# Python
ai/.venv/
ai/__pycache__/
**/*.pyc

# Node
web/node_modules/
web/dist/

# Env
.env
**/.env