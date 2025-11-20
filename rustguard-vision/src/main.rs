#[cfg(feature = "frontend")]
mod cxxqt_object;
pub mod video_pipeline;
pub mod db;

#[cfg(feature = "frontend")]
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    println!("RustGuard Vision Backend Initialized.");

    // Initialize Database (Mock connection for now as we might not have a live DB)
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // We spawn the database connection in the background or initialize it here.
    // For this example, we'll just log it.
    rt.spawn(async {
        if let Ok(_) = db::Database::connect("postgres://user:pass@localhost/rustguard").await {
             println!("Database connected successfully.");
        } else {
             println!("Database connection failed (expected in mock env).");
        }
    });

    #[cfg(feature = "frontend")]
    {
        let mut app = QGuiApplication::new();
        let mut engine = QQmlApplicationEngine::new();

        if let Some(engine) = engine.as_mut() {
            engine.load(&QUrl::from("qrc:/qml/main.qml"));
        }

        if let Some(app) = app.as_mut() {
            app.exec();
        }
    }

    #[cfg(not(feature = "frontend"))]
    {
        println!("Running in HEADLESS/NO-QT mode. Frontend features disabled.");
        // Keep the main thread alive for the async tasks to run in this mode
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
