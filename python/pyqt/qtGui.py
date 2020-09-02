import sys
import time
from PyQt5.QtWidgets import (QVBoxLayout, QHBoxLayout, QGridLayout, QPushButton, QDesktopWidget,
                             QLabel, QWidget, QMainWindow, QLineEdit, QApplication)
from PyQt5.QtCore import QDate, QSize, Qt, pyqtSignal, pyqtSlot, QObject
from PyQt5.QtGui import QIcon, QFont, QPixmap, QImage
import networkConnect


class MyNetwork(networkConnect.NetworkLogin):

    def connectWifi(self):
        self.connect()
        status = self.isConnected()
        if status == 1:
            self.login()
        elif status == 2:
            self.print('Congratulations~ Have a good time!')
        else:
            # self.connect()
            self.print(f'Cannot connect to {self.ssid}')
        # if count > 1:
        #     self.print('continue to try...')
        #     time.sleep(3)


class MySignals(QObject):

    connectWifi = pyqtSignal()


class MyWindow(QMainWindow):

    def __init__(self):
        super().__init__()
        self.initUI()

    def initUI(self):
        self.centralWidget = MyWiget(self)
        self.setCentralWidget(self.centralWidget)

        # self.setContentsMargins(5, 10, 5, 10)
        self.statusBar()
        self.resize(500, 750)
        self.setWindowTitle('Login')
        self.center()

    def center(self):
        qr = self.frameGeometry()
        cp = QDesktopWidget().availableGeometry().center()
        qr.moveCenter(cp)
        self.move(qr.topLeft())


class MyWiget(QWidget):

    def __init__(self, parent):
        super().__init__()
        self.parent = parent
        self.myNet = MyNetwork(ssid='Tsinghua', print=self.showInStatusBar)
        self.initUI()
        self.initSingals()

    def initUI(self):
        windowLayout = QVBoxLayout(self)

        gridLayout = QGridLayout()
        gridWidget = QWidget()
        gridWidget.setLayout(gridLayout)

        # calendar = QCalendarWidget()
        # calendar.setGridVisible(True)

        img = QImage('./thuTransparent.png')
        pixImg = QPixmap.fromImage(img.scaled(QSize(500, 500)))
        picture = QLabel()
        # thuPicture.setFixedSize(490, 490)
        # thuPicture.move(250, 250)
        # thuPicture.setScaledContents(True)
        picture.setPixmap(pixImg)

        hbox0 = QHBoxLayout()
        hbox0.addStretch(0.01)
        hbox0.addWidget(picture)
        hbox0.addStretch(0.01)
        hwgt0 = QWidget()
        hwgt0.setLayout(hbox0)

        usernameLable = QLabel('<b>Username</b>')
        passwordLable = QLabel('<b>Password</b>')
        self.username = QLineEdit()
        self.username.setToolTip('Please input your <b>username</b>!')
        self.password = QLineEdit()
        self.password.setEchoMode(QLineEdit.Password)
        self.password.setToolTip('Please input your <b>password</b>!')

        # QToolTip.setFont(QFont('SansSerif', 10))
        loginButton = QPushButton('login')
        loginButton.clicked.connect(self.buttonClicked)
        # loginButton.setCheckable(True)

        gridLayout.addWidget(usernameLable, 1, 0)
        gridLayout.addWidget(self.username, 1, 1)
        gridLayout.addWidget(passwordLable, 2, 0)
        gridLayout.addWidget(self.password, 2, 1)

        # windowLayout.addWidget(calendar)
        windowLayout.addWidget(hwgt0)
        windowLayout.addWidget(gridWidget)
        windowLayout.addWidget(loginButton)

    def initSingals(self):
        self.signals = MySignals()
        self.signals.connectWifi.connect(self.connectAndLogin)

    def showInStatusBar(self, *messages):
        message = ' '.join(messages)
        self.parent.statusBar().showMessage(message)

    def connectAndLogin(self):
        self.myNet.username, self.myNet.password = self.username.text(), self.password.text()
        self.myNet.connectWifi()

    def keyPressEvent(self, e):
        if e.key() == Qt.Key_Return:
            self.signals.connectWifi.emit()
        elif e.key() == Qt.Key_Escape:
            self.parent.close()
    
    def buttonClicked(self):
        self.signals.connectWifi.emit()


if __name__ == '__main__':

    app = QApplication(sys.argv)
    win = MyWindow()
    win.show()
    sys.exit(app.exec_())
