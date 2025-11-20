use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, sync::Arc};

use rust_embed::RustEmbed;

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

#[derive(RustEmbed)]
#[folder = "static"]
struct StaticAssets;

pub async fn start_server(port: u16) {
    let app_state = AppState {
        interpreter: Interpreter::default(),
    };

    let app = Router::new()
        .route("/api/run", post(run_code))
        .route("/health", get(health_check))
        .route("/", get(static_index))
        .route("/*path", get(static_handler))
        .with_state(Arc::new(app_state));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!(
        "🕉️  Paanini IDE server running at http://localhost:{}",
        port
    );
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

async fn static_index() -> Result<Response, StatusCode> {
    serve_asset("index.html")
}

async fn static_handler(Path(path): Path<String>) -> Result<Response, StatusCode> {
    let normalized = path.trim_start_matches('/');
    let asset_path = if normalized.is_empty() {
        "index.html"
    } else {
        normalized
    };

    match serve_asset(asset_path) {
        Ok(response) => Ok(response),
        Err(_) => serve_asset("index.html"),
    }
}

fn serve_asset(path: &str) -> Result<Response, StatusCode> {
    StaticAssets::get(path)
        .map(|content| build_asset_response(path, content.data))
        .ok_or(StatusCode::NOT_FOUND)
}

fn build_asset_response(path: &str, data: Cow<'static, [u8]>) -> Response {
    let body = Body::from(data.into_owned());
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(body)
        .unwrap()
}
