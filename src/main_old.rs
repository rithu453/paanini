use std::{fs, net::SocketAddr, path::PathBuf};

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir, trace::TraceLayer};

mod interpreter;
use interpreter::{Interpreter, RunResult};

#[derive(Clone)]
struct AppState {
    interpreter: Interpreter,
}

#[derive(Deserialize)]
struct RunRequest {
    code: String,
}

#[derive(Serialize)]
struct RunResponse {
    output: String,
    errors: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    if !args.is_empty() {
        let file = PathBuf::from(args.remove(0));
        let src = fs::read_to_string(&file)?;
        let mut interp = Interpreter::default();
        let res = interp.run(&src);
        print!("{}", res.output);
        if !res.errors.is_empty() {
            eprintln!("\nErrors:\n{}", res.errors.join("\n"));
            std::process::exit(1);
        }
        return Ok(());
    }

    let state = AppState { interpreter: Interpreter::default() };

    let api = Router::new()
        .route("/run", post(api_run))
        .with_state(state.clone());

    let app = Router::new()
        .nest("/api", api)
        .fallback_service(ServeDir::new("static").append_index_html_on_directories(true))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Paanini IDE running at http://localhost:8080");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn api_run(State(_state): State<AppState>, Json(req): Json<RunRequest>) -> impl IntoResponse {
    // Create a fresh interpreter per run to avoid cross-session leakage
    let mut interp = Interpreter::default();
    let RunResult { output, errors } = interp.run(&req.code);
    let body = RunResponse { output, errors };
    (StatusCode::OK, Json(body))
}
