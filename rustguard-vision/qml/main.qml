import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import com.rustguard 1.0

ApplicationWindow {
    visible: true
    width: 1280
    height: 720
    title: "RustGuard Vision (Qt Edition)"

    VideoBackend {
        id: backend
        onAlertPersonFound: {
            console.log("Alert: " + name + " (" + confidence + ")")
        }
    }

    header: TabBar {
        id: bar
        width: parent.width
        TabButton {
            text: "Dashboard"
        }
        TabButton {
            text: "Analyse"
        }
        TabButton {
            text: "Admin"
        }
    }

    StackLayout {
        anchors.fill: parent
        currentIndex: bar.currentIndex

        // Tab 1: Dashboard
        Item {
            GridLayout {
                anchors.fill: parent
                columns: 2

                Rectangle {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    color: "black"

                    Text {
                        anchors.centerIn: parent
                        text: "Video Feed Placeholder (Rust Backend Connected)"
                        color: "white"
                    }

                    // Overlay example
                    Rectangle {
                        x: 100; y: 100
                        width: 50; height: 50
                        color: "transparent"
                        border.color: "red"
                        border.width: 2
                        visible: true // Controlled by Rust
                        Text { text: "Person 95%"; color: "red"; y: -20 }
                    }

                    Button {
                        text: "Start Camera"
                        anchors.bottom: parent.bottom
                        anchors.horizontalCenter: parent.horizontalCenter
                        onClicked: backend.start_camera_stream("rtsp://mock-camera")
                    }
                }

                ColumnLayout {
                    Layout.preferredWidth: 300
                    Layout.fillHeight: true

                    Text {
                        text: "Live Detection Log"
                        font.bold: true
                        Layout.margins: 10
                    }

                    ListView {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        clip: true
                        model: ListModel {
                            ListElement { name: "Unknown"; time: "10:00:05" }
                            ListElement { name: "Admin"; time: "10:00:12" }
                        }
                        delegate: Text { text: name + " detected at " + time; leftPadding: 10 }
                    }
                }
            }
        }

        // Tab 2: Analysis
        AnalysisTab {}

        // Tab 3: Admin
        AdminTab {}
    }
}
