use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::interpreter::Interpreter;

#[derive(Clone)]
pub struct AppState {
    pub interpreter: Interpreter,
}

#[derive(Deserialize)]
pub struct RunRequest {
    pub code: String,
}

#[derive(Serialize)]
pub struct RunResponse {
    pub output: String,
    pub errors: Vec<String>,
}

pub async fn start_server(port: u16) {
    let app_state = AppState {
        interpreter: Interpreter::default(),
    };

    let app = Router::new()
        .route("/api/run", post(run_code))
        .route("/health", get(health_check))
        .nest_service("/", ServeDir::new("static"))
        .with_state(Arc::new(app_state));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("🕉️  Paanini IDE server running at http://localhost:{}", port);
    println!("📝 Open your browser to start coding in Sanskrit!");

    axum::serve(listener, app).await.unwrap();
}

async fn run_code(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RunRequest>,
) -> Result<Json<RunResponse>, StatusCode> {
    let mut interpreter = state.interpreter.clone();
    let result = interpreter.run(&payload.code);

    Ok(Json(RunResponse {
        output: result.output,
        errors: result.errors,
    }))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "paanini-ide",
        "version": "0.1.0"
    }))
}