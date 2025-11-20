use anyhow::Result;
use image::DynamicImage;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};

use async_trait::async_trait;

#[async_trait]
pub trait VideoSource: Send + Sync {
    async fn next_frame(&mut self) -> Result<Option<DynamicImage>>;
}

/// Simulates a video by repeating a single image
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
        // Simulate frame rate (10 FPS)
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
            info!("SingleImageSource: Processed {} frames", self.frame_count);
        }

        Ok(self.cached_image.clone())
    }
}

/// Simulates a video by playing a sequence of images from a folder
pub struct FolderVideoSource {
    folder_path: PathBuf,
    image_files: Vec<PathBuf>,
    current_index: usize,
    frame_count: u64,
}

impl FolderVideoSource {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let folder_path = path.as_ref().to_path_buf();
        let mut image_files = Vec::new();

        // Read directory
        if folder_path.is_dir() {
            for entry in std::fs::read_dir(&folder_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        let ext_str = ext.to_string_lossy().to_lowercase();
                        if ext_str == "jpg" || ext_str == "png" || ext_str == "jpeg" || ext_str == "webp" {
                            image_files.push(path);
                        }
                    }
                }
            }
        }

        // Sort to ensure sequence order (frame_1, frame_2, etc.)
        image_files.sort();

        if image_files.is_empty() {
            warn!("No images found in folder {:?}", folder_path);
        } else {
            info!("Found {} images in sequence folder", image_files.len());
        }

        Ok(Self {
            folder_path,
            image_files,
            current_index: 0,
            frame_count: 0,
        })
    }
}

#[async_trait]
impl VideoSource for FolderVideoSource {
    async fn next_frame(&mut self) -> Result<Option<DynamicImage>> {
        // Simulate frame rate
        sleep(Duration::from_millis(100)).await;

        if self.image_files.is_empty() {
            return Err(anyhow::anyhow!("No images in folder {:?}", self.folder_path));
        }

        let current_path = &self.image_files[self.current_index];
        let img = image::open(current_path)?;

        // Advance index, loop back if at end
        self.current_index = (self.current_index + 1) % self.image_files.len();
        self.frame_count += 1;

        if self.frame_count % 50 == 0 {
            info!("FolderSource: Processed {} frames (Looping sequence)", self.frame_count);
        }

        Ok(Some(img))
    }
}
