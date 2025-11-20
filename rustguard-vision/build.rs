#[cfg(feature = "frontend")]
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    #[cfg(feature = "frontend")]
    CxxQtBuilder::new()
        .qml_module(QmlModule {
            uri: "com.rustguard",
            rust_files: &["src/cxxqt_object.rs"],
            qml_files: &["qml/main.qml"],
            ..Default::default()
        })
        .build();
}
