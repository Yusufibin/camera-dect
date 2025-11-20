#[cxx_qt::bridge]
pub mod my_object {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject(qml_uri = "com.rustguard", qml_version = "1.0")]
    pub struct VideoBackend {
        #[qproperty]
        camera_count: i32,
        #[qproperty]
        last_detected_name: QString,
    }

    #[cxx_qt::qsignals(VideoBackend)]
    pub enum Signals {
        AlertPersonFound { name: QString, confidence: f32 },
    }

    impl qobject::VideoBackend {
        #[qinvokable]
        pub fn start_camera_stream(self: Pin<&mut Self>, url: QString) {
            let url_str = url.to_string();
            println!("Starting camera stream for: {}", url_str);

            let qt_thread = self.qt_thread();

            tokio::spawn(async move {
                use crate::video_pipeline::{MockVideoSource, MockFaceDetector};
                use tokio::sync::mpsc;
                use cxx_qt_lib::QString;

                let (tx, mut rx) = mpsc::channel(100);
                let source = MockVideoSource::new(url_str);
                let detector = MockFaceDetector;

                // Spawn capture thread
                tokio::spawn(async move {
                    source.stream(tx).await;
                });

                // Process frames
                while let Some(frame) = rx.recv().await {
                    let detections = detector.detect(&frame);
                    for d in detections {
                        println!("Detected: {} at ({}, {})", d.label, d.x, d.y);

                        let name = d.label.clone();
                        let conf = d.confidence;

                        // Signal the UI on the main thread
                        qt_thread.queue(move |mut qobject| {
                            let qname = QString::from(&name);
                            qobject.as_mut().set_last_detected_name(qname.clone());
                            qobject.as_mut().emit(Signals::AlertPersonFound {
                                name: qname,
                                confidence: conf
                            });
                        }).ok();
                    }
                }
            });
        }
    }
}
