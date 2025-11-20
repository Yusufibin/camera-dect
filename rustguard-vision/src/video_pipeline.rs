use tokio::sync::mpsc;
use std::time::Duration;

pub struct Frame {
    pub id: u64,
    pub data: Vec<u8>, // Mock data
}

pub struct Detection {
    pub label: String,
    pub confidence: f32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub struct MockVideoSource {
    url: String,
}

impl MockVideoSource {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn stream(&self, sender: mpsc::Sender<Frame>) {
        let mut count = 0;
        loop {
            tokio::time::sleep(Duration::from_millis(33)).await; // ~30 FPS
            let frame = Frame {
                id: count,
                data: vec![0; 1024], // Dummy data
            };
            if sender.send(frame).await.is_err() {
                break;
            }
            count += 1;
        }
    }
}

pub struct MockFaceDetector;

impl MockFaceDetector {
    pub fn detect(&self, _frame: &Frame) -> Vec<Detection> {
        // Simulate detection occasionally
        if _frame.id % 30 == 0 {
             vec![Detection {
                label: "Person".to_string(),
                confidence: 0.95,
                x: 100.0,
                y: 100.0,
                w: 50.0,
                h: 50.0,
            }]
        } else {
            vec![]
        }
    }
}
