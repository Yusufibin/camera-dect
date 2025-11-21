This project is a Python port of the RustGuard Vision application.

## Structure

- `src/`: Contains the Python source code.
  - `main.py`: Entry point.
  - `video_pipeline.py`: Handles video ingestion (OpenCV) and face detection.
  - `database.py`: Handles SQLite database interactions.
- `qml/`: Contains the Qt Quick (QML) UI files.
- `rustguard.spec`: Configuration file for PyInstaller.
- `setup.iss`: Configuration file for Inno Setup.

## Requirements

- Python 3.8+
- `pip install -r requirements.txt`

## Running Development Version

Run the application from the `version_python` directory:

```bash
cd src
python main.py
```

## Building for Windows (Create Installer)

To create a Windows executable and installer, you need to perform these steps on a Windows machine.

### 1. Install Build Tools

Install **PyInstaller** via pip:

```bash
pip install pyinstaller
```

Install **Inno Setup** from [jrsoftware.org](https://jrsoftware.org/isdl.php).

### 2. Build Executable

From the `version_python` directory, run PyInstaller using the provided spec file:

```bash
pyinstaller rustguard.spec
```

This will create a `dist/RustGuard` directory containing the executable and dependencies.

### 3. Create Installer

1. Open `setup.iss` with Inno Setup Compiler.
2. Verify the paths in the script (especially if you are not running it from the `version_python` root).
3. Click **Build > Compile** (or press Ctrl+F9).

The installer (`RustGuard_Setup.exe`) will be generated in the `Output` folder (or inside `version_python` depending on default settings).

## Features

- **Live Video Feed**: Uses OpenCV to capture webcam (or falls back to a mock stream if no camera is found).
- **Face Detection**: Uses OpenCV Haar Cascades to detect faces.
- **Database**: Logs detections to a local SQLite database (`rustguard.db`).
- **GUI**: Modern Qt Quick interface using PySide6.

## Notes

- The frontend communicates with the Python backend via PySide6 Signals and Slots.
- Video frames are passed to QML via a `QQuickImageProvider`.
