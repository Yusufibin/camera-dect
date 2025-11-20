use rustguard_vision::video_pipeline::{MockVideoSource, MockFaceDetector};
use tokio::sync::mpsc;
use std::time::Duration;

#[tokio::test]
async fn test_pipeline_flow() {
    // Setup
    let (tx, mut rx) = mpsc::channel(10);
    let source = MockVideoSource::new("rtsp://test".to_string());
    let detector = MockFaceDetector;

    // Spawn capture
    tokio::spawn(async move {
        source.stream(tx).await;
    });

    // Verify we get frames and can run detection
    // We'll check the first 35 frames to ensure we hit the "30th frame" trigger in the mock
    let mut detected_count = 0;
    let mut frame_count = 0;

    while let Some(frame) = rx.recv().await {
        frame_count += 1;
        let detections = detector.detect(&frame);
        if !detections.is_empty() {
            detected_count += 1;
            assert_eq!(detections[0].label, "Person");
        }

        if frame_count >= 35 {
            break;
        }
    }

    assert!(frame_count >= 30);
    assert!(detected_count > 0, "Should have detected at least one person");
}
