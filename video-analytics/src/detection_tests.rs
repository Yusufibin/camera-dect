#[cfg(test)]
mod tests {
    use crate::detection::{FaceDetector, MockFaceDetector};
    use image::{ImageBuffer, Rgb};

    #[tokio::test]
    async fn test_mock_face_detector() {
        let detector = MockFaceDetector;
        let img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(100, 100);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        let detections = detector.detect(&dynamic_img).await.unwrap();
        assert_eq!(detections.len(), 1);
        assert_eq!(detections[0].score, 0.95);
    }
}
