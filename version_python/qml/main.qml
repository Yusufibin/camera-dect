import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

ApplicationWindow {
    visible: true
    width: 1280
    height: 720
    title: "RustGuard Vision (Python Edition)"

    // Connect to the backend signals provided via context property
    Connections {
        target: backend
        function onAlertPersonFound(name, confidence) {
            console.log("Alert: " + name + " (" + confidence + ")")
            logModel.append({"name": name, "time": new Date().toLocaleTimeString()})
        }
        function onFrameReady() {
            videoOutput.source = "image://opencv/live?id=" + Math.random()
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

                    Image {
                        id: videoOutput
                        anchors.fill: parent
                        fillMode: Image.PreserveAspectFit
                        source: "" // Set by onFrameReady
                        cache: false
                    }

                    Text {
                        anchors.centerIn: parent
                        text: "Waiting for stream..."
                        color: "white"
                        visible: videoOutput.status !== Image.Ready
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
                            id: logModel
                            // Initial dummy data
                            ListElement { name: "System"; time: "Started" }
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
