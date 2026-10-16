import QtQuick 2.15
import SddmComponents 2.0

Rectangle {
    id: root
    width: 1920
    height: 1080
    color: "#030509"
    property color pink: "#ff2a91"
    property color cyan: "#18f3ec"
    property color yellow: "#ffe94a"
    property color green: "#59ff83"
    property int sessionIndex: session.index
    property string message: "INSERT COIN / ENTER PASSCODE TO START"
    property color messageColor: yellow

    TextConstants { id: textConstants }

    Image {
        id: backgroundArtwork
        anchors.fill: parent
        source: "login_background.jpeg"
        fillMode: Image.PreserveAspectCrop
        asynchronous: false
        smooth: true
    }

    Connections {
        target: sddm
        function onLoginSucceeded() {
            root.message = textConstants.loginSucceeded
            root.messageColor = root.green
        }
        function onLoginFailed() {
            password.text = ""
            root.message = textConstants.loginFailed
            root.messageColor = root.pink
            password.forceActiveFocus()
        }
    }

    Timer {
        interval: 1000; running: true; repeat: true
        onTriggered: clock.text = "◴ TIME: " + Qt.formatTime(new Date(), "hh:mm:ss")
    }
    // The supplied artwork already has static HUD labels in the corners.
    // Keep the clock dynamic but place it within the upper HUD area.
    Text {
        id: clock
        x: 30; y: 28
        text: "◴ TIME: " + Qt.formatTime(new Date(), "hh:mm:ss")
        color: root.yellow
        font.family: "monospace"; font.pixelSize: 18; font.bold: true
    }

    Text {
        anchors.horizontalCenter: parent.horizontalCenter
        y: 72
        width: Math.min(900, root.width - 80)
        horizontalAlignment: Text.AlignHCenter
        elide: Text.ElideRight
        text: "MIGHTY ACTION X"
        color: root.yellow
        style: Text.Outline; styleColor: root.pink
        font.family: "monospace"; font.pixelSize: 52; font.bold: true
    }
    Text {
        anchors.horizontalCenter: parent.horizontalCenter
        y: 137
        width: Math.min(760, root.width - 80)
        horizontalAlignment: Text.AlignHCenter
        elide: Text.ElideRight
        text: "CYBER-MEDICAL HUD EDITION"
        color: root.cyan
        font.family: "monospace"; font.pixelSize: 18; font.bold: true
    }
    Text {
        anchors.horizontalCenter: parent.horizontalCenter
        y: 220
        width: Math.min(1000, root.width - 80)
        horizontalAlignment: Text.AlignHCenter
        wrapMode: Text.WordWrap
        text: "✿  INSERT COIN / ENTER PASSCODE TO START  ✿"
        color: root.yellow
        font.family: "monospace"; font.pixelSize: 23; font.bold: true
    }

    Rectangle {
        id: loginFrame
        width: Math.min(820, root.width - 64)
        height: 480
        anchors.verticalCenterOffset: 68
        anchors.centerIn: parent
        color: "#e6050710"
        border.width: 5
        border.color: root.pink
        radius: 22
        Rectangle {
            anchors.fill: parent
            anchors.margins: 10
            color: "transparent"
            border.width: 3
            border.color: root.cyan
            radius: 14
        }
        Column {
            anchors.fill: parent
            anchors.margins: 30
            spacing: 12
            Text {
                text: "SELECT PLAYER"
                color: root.pink
                font.family: "monospace"; font.pixelSize: 24; font.bold: true
                anchors.horizontalCenter: parent.horizontalCenter
            }
            Text {
                text: "PLAYER 01: " + (userModel.lastUser || "PLAYER")
                color: "#ffffff"
                font.family: "monospace"; font.pixelSize: 19; font.bold: true
                anchors.horizontalCenter: parent.horizontalCenter
            }
            TextBox {
                id: username
                width: parent.width; height: 46
                text: userModel.lastUser
                font.family: "monospace"; font.pixelSize: 18
                textColor: "#ffffff"
                radius: 6; borderColor: root.cyan; focusColor: root.pink; hoverColor: root.yellow
                color: "#080d16"
                KeyNavigation.tab: password
                Keys.onReturnPressed: password.forceActiveFocus()
                Keys.onEnterPressed: password.forceActiveFocus()
            }
            Text {
                text: "🔒 ENTER PASSCODE / INSERT GASHAT"
                width: parent.width
                horizontalAlignment: Text.AlignHCenter
                wrapMode: Text.WordWrap
                color: "#ffffff"
                font.family: "monospace"; font.pixelSize: 17; font.bold: true
                anchors.horizontalCenter: parent.horizontalCenter
            }
            PasswordBox {
                id: password
                width: parent.width; height: 46
                font.family: "monospace"; font.pixelSize: 19
                textColor: root.pink; color: "#080d16"
                radius: 6; borderColor: root.cyan; focusColor: root.pink; hoverColor: root.yellow
                KeyNavigation.backtab: username
                KeyNavigation.tab: session
                Keys.onReturnPressed: root.tryLogin()
                Keys.onEnterPressed: root.tryLogin()
            }
            ComboBox {
                id: session
                width: parent.width; height: 50
                model: sessionModel
                index: sessionModel.lastIndex
                font.family: "monospace"; font.pixelSize: 16; font.bold: true
                color: "#09131b"; borderColor: root.cyan; focusColor: root.yellow
                hoverColor: "#17424b"; textColor: "#ffffff"; menuColor: "#09131b"
                borderWidth: 2
                arrowIcon: ""
                rowDelegate: Component {
                    Rectangle {
                        width: session.width
                        height: 42
                        color: ListView.isCurrentItem ? "#16363b" : "#09131b"
                        Text {
                            anchors.fill: parent
                            anchors.leftMargin: 14
                            anchors.rightMargin: 38
                            verticalAlignment: Text.AlignVCenter
                            elide: Text.ElideRight
                            text: modelItem.name
                            color: root.cyan
                            font: session.font
                        }
                    }
                }
                Text {
                    anchors.right: parent.right
                    anchors.rightMargin: 12
                    anchors.verticalCenter: parent.verticalCenter
                    text: "▼"
                    color: root.yellow
                    font.pixelSize: 13
                    z: 2
                }
                KeyNavigation.backtab: password
            }
            Row {
                spacing: 16
                anchors.horizontalCenter: parent.horizontalCenter
                Button {
                    id: continueButton
                    width: 230; height: 54
                    text: "✿  CONTINUE"
                    color: "#351020"; borderColor: root.pink
                    activeColor: "#842052"; pressedColor: "#d21d72"
                    disabledColor: "#26202a"; textColor: "#ffffff"
                    font.family: "monospace"; font.pixelSize: 17; font.bold: true
                    onClicked: root.tryLogin()
                    KeyNavigation.tab: retryButton
                }
                Button {
                    id: retryButton
                    width: 190; height: 54
                    text: "↻  RETRY"
                    color: "#08272d"; borderColor: root.cyan
                    activeColor: "#075b62"; pressedColor: "#0aa6a8"
                    disabledColor: "#26202a"; textColor: "#ffffff"
                    font.family: "monospace"; font.pixelSize: 17; font.bold: true
                    onClicked: { password.text = ""; password.forceActiveFocus() }
                }
            }
            Text {
                text: root.message
                color: root.messageColor
                width: parent.width
                horizontalAlignment: Text.AlignHCenter
                wrapMode: Text.WordWrap
                elide: Text.ElideNone
                font.family: "monospace"; font.pixelSize: 14; font.bold: true
                anchors.horizontalCenter: parent.horizontalCenter
            }
        }
    }

    Row {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.bottom: parent.bottom
        anchors.bottomMargin: 42
        spacing: 18
        Button {
            width: 190; height: 46
            text: "⏻  POWER OFF"
            color: "#27101b"; borderColor: root.pink; activeColor: "#70203f"
            pressedColor: "#a42558"; disabledColor: "#211d24"; textColor: "#ffffff"
            font.family: "monospace"; font.pixelSize: 14; font.bold: true
            enabled: sddm.canPowerOff; onClicked: sddm.powerOff()
        }
        Button {
            width: 170; height: 46
            text: "↻  REBOOT"
            color: "#082329"; borderColor: root.cyan; activeColor: "#07515a"
            pressedColor: "#078991"; disabledColor: "#211d24"; textColor: "#ffffff"
            font.family: "monospace"; font.pixelSize: 14; font.bold: true
            enabled: sddm.canReboot; onClicked: sddm.reboot()
        }
    }

    Text {
        x: 18; anchors.bottom: parent.bottom; anchors.bottomMargin: 12
        text: "SELECT PLAYER  //  SYSTEM_SECURE_LOCK"
        color: "#75aaa8"; font.family: "monospace"; font.pixelSize: 12
    }
    Text {
        anchors.right: parent.right; anchors.bottom: parent.bottom
        anchors.rightMargin: 18; anchors.bottomMargin: 12
        text: "INSERT GASHAT TO HENSHIN"
        color: "#75aaa8"; font.family: "monospace"; font.pixelSize: 12
        horizontalAlignment: Text.AlignRight
    }

    function tryLogin() {
        root.message = "AUTHENTICATING PLAYER..."
        root.messageColor = root.cyan
        sddm.login(username.text, password.text, root.sessionIndex)
    }

    Component.onCompleted: {
        if (username.text.length > 0) password.forceActiveFocus()
        else username.forceActiveFocus()
    }
}