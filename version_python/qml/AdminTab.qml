import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Page {
    title: "Administration"

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 20
        spacing: 20

        GroupBox {
            title: "Configuration Caméras"
            Layout.fillWidth: true

            ColumnLayout {
                anchors.fill: parent
                TextField {
                    placeholderText: "RTSP URL 1"
                    Layout.fillWidth: true
                    text: "rtsp://192.168.1.10:554/stream"
                }
                TextField {
                    placeholderText: "RTSP URL 2"
                    Layout.fillWidth: true
                }
                Button {
                    text: "Sauvegarder"
                }
            }
        }

        GroupBox {
            title: "Moniteur Système"
            Layout.fillWidth: true
            Layout.fillHeight: true

            RowLayout {
                anchors.centerIn: parent
                spacing: 50

                // Mock CPU Gauge
                Rectangle {
                    width: 100; height: 100
                    radius: 50
                    color: "transparent"
                    border.color: "#00aa00"
                    border.width: 5
                    Text { anchors.centerIn: parent; text: "CPU\n15%"; horizontalAlignment: Text.AlignHCenter }
                }

                // Mock RAM Gauge
                Rectangle {
                    width: 100; height: 100
                    radius: 50
                    color: "transparent"
                    border.color: "#0000aa"
                    border.width: 5
                    Text { anchors.centerIn: parent; text: "RAM\n4.2GB"; horizontalAlignment: Text.AlignHCenter }
                }
            }
        }
    }
}
