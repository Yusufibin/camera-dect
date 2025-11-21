import sqlite3
import datetime
import os
import sys
from typing import List, Tuple, Optional
from PySide6.QtCore import QStandardPaths

class Database:
    def __init__(self, db_name: str = "rustguard.db"):
        # Determine a writable path for the database
        # Use QStandardPaths to find the AppData/Local data location for this app
        # On Windows: C:/Users/<User>/AppData/Local/RustGuard/
        # On Linux: ~/.local/share/RustGuard/
        writable_dir = QStandardPaths.writableLocation(QStandardPaths.AppLocalDataLocation)

        # QStandardPaths might return empty if app name isn't set globally in QApp,
        # but we can construct it manually using APPDATA or XDG_DATA_HOME if needed.
        # PySide6 QStandardPaths usually works if QApp is created, but this class might be instantiated before QApp?
        # No, in main.py QApp is created first. But wait, VideoWorker instantiates Database inside thread.

        # Fallback if QStandardPaths is tricky or QApp not ready (though usually it is):
        if not writable_dir:
            if sys.platform == 'win32':
                app_data = os.getenv('LOCALAPPDATA') or os.getenv('APPDATA')
                writable_dir = os.path.join(app_data, 'RustGuard')
            else:
                # Linux/Mac
                home = os.path.expanduser("~")
                writable_dir = os.path.join(home, ".local", "share", "RustGuard")

        # Ensure directory exists
        if not os.path.exists(writable_dir):
            try:
                os.makedirs(writable_dir)
            except OSError as e:
                print(f"Error creating data directory {writable_dir}: {e}")
                # Fallback to temp or current dir if we really fail
                writable_dir = "."

        self.db_path = os.path.join(writable_dir, db_name)
        print(f"Database path: {self.db_path}")
        self._init_db()

    def _init_db(self):
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute("""
            CREATE TABLE IF NOT EXISTS visits (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                visitor_name TEXT NOT NULL,
                confidence REAL NOT NULL,
                visited_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        """)
        conn.commit()
        conn.close()

    def add_visit(self, name: str, confidence: float):
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute("INSERT INTO visits (visitor_name, confidence) VALUES (?, ?)", (name, confidence))
        conn.commit()
        conn.close()

    def get_visits(self, limit: int = 50) -> List[Tuple]:
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute("SELECT id, visitor_name, confidence, visited_at FROM visits ORDER BY visited_at DESC LIMIT ?", (limit,))
        rows = cursor.fetchall()
        conn.close()
        return rows
