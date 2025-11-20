use anyhow::Result;
use image::DynamicImage;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

use async_trait::async_trait;

#[async_trait]
pub trait VideoSource: Send + Sync {
    async fn next_frame(&mut self) -> Result<Option<DynamicImage>>;
}

pub struct MockVideoSource {
    image_path: PathBuf,
    cached_image: Option<DynamicImage>,
    frame_count: u64,
}

impl MockVideoSource {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            image_path: path.as_ref().to_path_buf(),
            cached_image: None,
            frame_count: 0,
        }
    }
}

#[async_trait]
impl VideoSource for MockVideoSource {
    async fn next_frame(&mut self) -> Result<Option<DynamicImage>> {
        // Simulate frame rate
        sleep(Duration::from_millis(100)).await;

        if self.cached_image.is_none() {
            if !self.image_path.exists() {
                return Err(anyhow::anyhow!("Mock image not found at {:?}", self.image_path));
            }
            let img = image::open(&self.image_path)?;
            self.cached_image = Some(img);
        }

        self.frame_count += 1;

        if self.frame_count % 50 == 0 {
            info!("Processed {} frames from mock source", self.frame_count);
        }

        // Clone the cached image to simulate a new frame
        Ok(self.cached_image.clone())
    }
}
