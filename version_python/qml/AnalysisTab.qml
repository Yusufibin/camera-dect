import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Page {
    title: "Analyse de Fréquentation"

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 10

        RowLayout {
            Layout.fillWidth: true
            Button { text: "Aujourd'hui"; highlighted: true }
            Button { text: "7 derniers jours" }
            Button { text: "Clients VIP" }
            Item { Layout.fillWidth: true } // Spacer
        }

        ListView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            model: ListModel {
                ListElement { name: "John Doe"; date: "2023-10-27 10:00"; type: "VIP" }
                ListElement { name: "Jane Smith"; date: "2023-10-27 10:15"; type: "Visitor" }
                ListElement { name: "Bob Johnson"; date: "2023-10-27 10:30"; type: "Staff" }
            }
            delegate: Rectangle {
                width: parent.width
                height: 50
                color: index % 2 == 0 ? "#eee" : "white"
                border.color: "#ccc"
                border.width: 1

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 10
                    Text { text: name; Layout.preferredWidth: 150 }
                    Text { text: date; Layout.preferredWidth: 150 }
                    Text { text: type; Layout.preferredWidth: 100 }
                    Item { Layout.fillWidth: true }
                    Button { text: "Détails" }
                }
            }
        }
    }
}
