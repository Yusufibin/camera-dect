import cv2
import numpy as np
import threading
import time
import os
import sys
from typing import Optional, List, Dict

class Frame:
    def __init__(self, data: np.ndarray, frame_id: int):
        self.data = data
        self.id = frame_id

class Detection:
    def __init__(self, label: str, confidence: float, x: int, y: int, w: int, h: int):
        self.label = label
        self.confidence = confidence
        self.x = x
        self.y = y
        self.w = w
        self.h = h

class VideoSource:
    def __init__(self, source=0):
        self.source = source
        self.cap = cv2.VideoCapture(self.source)
        self.running = False

    def get_frame(self) -> Optional[np.ndarray]:
        if not self.cap.isOpened():
            # Fallback to mock if camera not available
            return self._generate_mock_frame()

        ret, frame = self.cap.read()
        if not ret:
            return self._generate_mock_frame()
        return frame

    def _generate_mock_frame(self) -> np.ndarray:
        # Create a noise frame or black frame with text
        img = np.zeros((720, 1280, 3), dtype=np.uint8)
        cv2.putText(img, "No Signal / Mock Frame", (400, 360), cv2.FONT_HERSHEY_SIMPLEX, 2, (255, 255, 255), 3)
        # Add some noise
        noise = np.random.randint(0, 256, (720, 1280, 3), dtype=np.uint8)
        img = cv2.addWeighted(img, 0.8, noise, 0.2, 0)
        time.sleep(0.03) # Simulate 30fps
        return img

    def release(self):
        if self.cap.isOpened():
            self.cap.release()

class FaceDetector:
    def __init__(self):
        # Locate cascade file
        cascade_path = None

        # Check if bundled in PyInstaller
        if hasattr(sys, '_MEIPASS'):
            bundled_path = os.path.join(sys._MEIPASS, 'haarcascade_frontalface_default.xml')
            if os.path.exists(bundled_path):
                cascade_path = bundled_path

        # Fallback to system opencv data
        if not cascade_path:
             cascade_path = cv2.data.haarcascades + 'haarcascade_frontalface_default.xml'

        if cascade_path and os.path.exists(cascade_path):
            self.face_cascade = cv2.CascadeClassifier(cascade_path)
            self.mock_mode = False
        else:
            print(f"Warning: Cascade file not found at {cascade_path}. Using mock detector.")
            self.mock_mode = True

    def detect(self, frame: np.ndarray) -> List[Detection]:
        if self.mock_mode:
            # Mock detection every 30 frames logic handled by caller usually,
            # but here we just return random detection occasionally
            if np.random.rand() > 0.95:
                return [Detection("Person (Mock)", 0.98, 100, 100, 200, 200)]
            return []

        gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
        faces = self.face_cascade.detectMultiScale(gray, 1.1, 4)

        detections = []
        for (x, y, w, h) in faces:
            # Confidence is not provided by detectMultiScale directly, we fake it based on size or just 1.0
            detections.append(Detection("Person", 0.95, int(x), int(y), int(w), int(h)))

        return detections
