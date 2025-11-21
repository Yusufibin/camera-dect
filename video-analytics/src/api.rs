use axum::{
    extract::State,
    routing::get,
    Router,
    Json,
    response::{Html, IntoResponse},
};
use serde_json::json;
use tokio::sync::broadcast;
use crate::pipeline::DetectionEvent;

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<DetectionEvent>,
}

pub fn app(tx: broadcast::Sender<DetectionEvent>) -> Router {
    let state = AppState { tx };

    Router::new()
        .route("/", get(index))
        .route("/status", get(status))
        .route("/events", get(events_handler))
        .with_state(state)
}

async fn index() -> Html<&'static str> {
    Html("<h1>Video Analytics Backend</h1><p><a href='/events'>Connect to Event Stream (Server-Sent Events logic needed)</a></p>")
}

async fn status() -> Json<serde_json::Value> {
    Json(json!({ "status": "running", "uptime": "unknown" }))
}

// Basic handler to show how we might expose events
// In a real app, this would be a WebSocket or SSE endpoint
async fn events_handler(State(state): State<AppState>) -> impl IntoResponse {
    let mut rx = state.tx.subscribe();

    // This is a blocking wait for one event just to demonstrate
    // In reality, use SSE
    match rx.recv().await {
        Ok(event) => Json(json!(event)),
        Err(_) => Json(json!({ "error": "lagged or closed" })),
    }
}
