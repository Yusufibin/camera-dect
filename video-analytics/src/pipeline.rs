use anyhow::Result;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{info, error};
use crate::ingestion::VideoSource;
use crate::detection::{FaceDetector, BoundingBox};
use crate::storage::Storage;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct DetectionEvent {
    pub camera_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub detections: Vec<BoundingBox>,
    // We don't send the full image over broadcast to save bandwidth, usually just metadata
    // But for the pipeline, we pass it along.
}

pub struct Pipeline {
    video_source: Box<dyn VideoSource>,
    detector: Box<dyn FaceDetector>,
    storage: Arc<dyn Storage>,
    camera_id: String,
    broadcaster: broadcast::Sender<DetectionEvent>,
}

impl Pipeline {
    pub fn new(
        video_source: Box<dyn VideoSource>,
        detector: Box<dyn FaceDetector>,
        storage: Arc<dyn Storage>,
        camera_id: String,
        broadcaster: broadcast::Sender<DetectionEvent>,
    ) -> Self {
        Self {
            video_source,
            detector,
            storage,
            camera_id,
            broadcaster,
        }
    }

    pub async fn run(mut self) -> Result<()> {
        info!("Starting pipeline for camera {}", self.camera_id);

        loop {
            match self.video_source.next_frame().await {
                Ok(Some(frame)) => {
                    // 1. Detect Faces
                    match self.detector.detect(&frame).await {
                        Ok(detections) => {
                            if !detections.is_empty() {
                                info!("Detected {} faces", detections.len());

                                // 2. Store metadata (and possibly image)
                                let event = DetectionEvent {
                                    camera_id: self.camera_id.clone(),
                                    timestamp: chrono::Utc::now(),
                                    detections: detections.clone(),
                                };

                                if let Err(e) = self.storage.save_event(&event, &frame).await {
                                    error!("Failed to save event: {}", e);
                                }

                                // 3. Broadcast event (for frontend)
                                if let Err(_) = self.broadcaster.send(event) {
                                    // No listeners
                                }
                            }
                        }
                        Err(e) => {
                            error!("Detection error: {}", e);
                        }
                    }
                }
                Ok(None) => {
                    info!("Stream ended for camera {}", self.camera_id);
                    break;
                }
                Err(e) => {
                    error!("Video source error: {}", e);
                    // Determine if we should break or continue
                    // For now, wait a bit and retry
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
        Ok(())
    }
}
