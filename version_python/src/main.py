import sys
import os
import threading
import time
import cv2
import numpy as np
from PySide6.QtCore import QObject, Signal, Slot, Property, QUrl, QTimer, QSize, QThread
from PySide6.QtGui import QImage, QPainter, QColor
from PySide6.QtQml import QQmlApplicationEngine
from PySide6.QtWidgets import QApplication
from PySide6.QtQuick import QQuickImageProvider

from video_pipeline import VideoSource, FaceDetector
from database import Database

# Helper to find resources in both dev and frozen (PyInstaller) modes
def resource_path(relative_path):
    """ Get absolute path to resource, works for dev and for PyInstaller """
    if hasattr(sys, '_MEIPASS'):
        # PyInstaller creates a temp folder and stores path in _MEIPASS
        return os.path.join(sys._MEIPASS, relative_path)
    return os.path.join(os.path.abspath("."), relative_path)

# Image Provider to feed OpenCV frames to QML
class OpenCVImageProvider(QQuickImageProvider):
    def __init__(self):
        super().__init__(QQuickImageProvider.ImageType.Image)
        self.current_image = QImage(1280, 720, QImage.Format.Format_RGB888)
        self.current_image.fill(QColor("black"))

    def requestImage(self, id, size, requestedSize):
        return self.current_image, self.current_image.size()

    def update_image(self, img):
        if img is None:
            return

        # Convert BGR (OpenCV) to RGB (Qt)
        height, width, channel = img.shape
        bytes_per_line = 3 * width
        # We need to keep a reference to the data or copy it. QImage(data, ...) references buffer.
        # .copy() ensures we own the data.
        rgb_image = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)
        q_image = QImage(rgb_image.data, width, height, bytes_per_line, QImage.Format.Format_RGB888).copy()
        self.current_image = q_image

class VideoWorker(QObject):
    frameProcessed = Signal(np.ndarray)
    alertGenerated = Signal(str, float)

    def __init__(self, source_url):
        super().__init__()
        self.source_url = source_url
        self.running = False
        self.video_source = None
        self.detector = None
        self.db = None
        self.frame_count = 0

    @Slot()
    def start(self):
        # Initialize resources in the worker thread
        source = 0 if "rtsp" not in self.source_url else self.source_url
        self.video_source = VideoSource(source)
        self.detector = FaceDetector()
        self.db = Database()
        self.running = True

        while self.running:
            frame = self.video_source.get_frame()
            if frame is None:
                time.sleep(0.1)
                continue

            self.frame_count += 1

            # Detection
            detections = self.detector.detect(frame)

            for det in detections:
                cv2.rectangle(frame, (det.x, det.y), (det.x+det.w, det.y+det.h), (0, 0, 255), 2)
                cv2.putText(frame, f"{det.label} {int(det.confidence*100)}%", (det.x, det.y-10),
                            cv2.FONT_HERSHEY_SIMPLEX, 0.9, (0, 0, 255), 2)

                if self.frame_count % 30 == 0:
                     self.alertGenerated.emit(det.label, det.confidence)
                     # DB write should ideally be async or separate, but sqlite is fast enough here
                     try:
                        self.db.add_visit(det.label, det.confidence)
                     except Exception as e:
                         print(f"DB Error: {e}")

            self.frameProcessed.emit(frame)

            # Simple FPS throttle
            time.sleep(0.03)

    @Slot()
    def stop(self):
        self.running = False
        if self.video_source:
            self.video_source.release()

class VideoBackend(QObject):
    alertPersonFound = Signal(str, float, arguments=['name', 'confidence'])
    frameReady = Signal()

    def __init__(self, image_provider):
        super().__init__()
        self.image_provider = image_provider
        self.worker_thread = None
        self.worker = None

    @Slot(str)
    def start_camera_stream(self, url):
        if self.worker_thread and self.worker_thread.isRunning():
            return

        print(f"Starting camera stream: {url}")
        self.worker_thread = QThread()
        self.worker = VideoWorker(url)
        self.worker.moveToThread(self.worker_thread)

        # Connect signals
        self.worker_thread.started.connect(self.worker.start)
        self.worker.frameProcessed.connect(self.handle_frame)
        self.worker.alertGenerated.connect(self.alertPersonFound)

        # Clean up
        self.worker_thread.finished.connect(self.worker.stop)
        self.worker_thread.finished.connect(self.worker_thread.deleteLater)

        self.worker_thread.start()

    @Slot(np.ndarray)
    def handle_frame(self, frame):
        self.image_provider.update_image(frame)
        self.frameReady.emit()

    @Slot()
    def stop_stream(self):
        if self.worker:
            self.worker.stop()
        if self.worker_thread:
            self.worker_thread.quit()
            self.worker_thread.wait()

def main():
    app = QApplication(sys.argv)
    engine = QQmlApplicationEngine()

    image_provider = OpenCVImageProvider()
    engine.addImageProvider("opencv", image_provider)

    backend = VideoBackend(image_provider)
    engine.rootContext().setContextProperty("backend", backend)

    # Determine QML path based on mode
    if hasattr(sys, '_MEIPASS'):
        # Frozen mode: qml/ is at root of bundle
        qml_file = os.path.join(sys._MEIPASS, "qml", "main.qml")
    else:
        # Dev mode: qml/ is relative to src/
        qml_file = os.path.join(os.path.dirname(__file__), "../qml/main.qml")

    qml_file = os.path.abspath(qml_file)

    if not os.path.exists(qml_file):
        print(f"Error: QML file not found at {qml_file}")
        # Try fallback for different working directory contexts
        fallback = os.path.join(os.getcwd(), "qml/main.qml")
        if os.path.exists(fallback):
             qml_file = fallback
        else:
             sys.exit(-1)

    engine.load(QUrl.fromLocalFile(qml_file))

    if not engine.rootObjects():
        sys.exit(-1)

    sys.exit(app.exec())

if __name__ == "__main__":
    main()
