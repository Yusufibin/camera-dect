mod ingestion;
mod detection;
mod pipeline;
mod storage;
mod api;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::ingestion::MockVideoSource;
use crate::detection::MockFaceDetector;
use crate::pipeline::{Pipeline, DetectionEvent};
use crate::storage::MockStorage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting Video Analytics Backend...");

    // Create broadcast channel for events (Capacity 100)
    let (tx, _rx) = broadcast::channel::<DetectionEvent>(100);

    // Setup Pipeline Components
    let video_source = Box::new(MockVideoSource::new("assets/sample_frame.jpg"));
    let detector = Box::new(MockFaceDetector);
    let storage = Arc::new(MockStorage);

    let pipeline = Pipeline::new(
        video_source,
        detector,
        storage,
        "camera_01".to_string(),
        tx.clone(),
    );

    // Spawn Pipeline Task
    tokio::spawn(async move {
        if let Err(e) = pipeline.run().await {
            eprintln!("Pipeline error: {}", e);
        }
    });

    // Setup API Server
    let app = api::app(tx);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
