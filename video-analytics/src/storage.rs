use anyhow::Result;
use async_trait::async_trait;
use image::DynamicImage;
use crate::pipeline::DetectionEvent;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_event(&self, event: &DetectionEvent, image: &DynamicImage) -> Result<()>;
}

pub struct MockStorage;

#[async_trait]
impl Storage for MockStorage {
    async fn save_event(&self, _event: &DetectionEvent, _image: &DynamicImage) -> Result<()> {
        // Here we would save to DB and disk
        // For now, just log
        // tracing::debug!("Saved event for camera {} at {}", event.camera_id, event.timestamp);
        Ok(())
    }
}
