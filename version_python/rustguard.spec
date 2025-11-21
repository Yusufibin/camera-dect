# -*- mode: python ; coding: utf-8 -*-

import sys
import os
import cv2
from PySide6 import QtCore

# Locate the QML directory
qml_dir = os.path.abspath(os.path.join(os.getcwd(), 'qml'))
src_dir = os.path.abspath(os.path.join(os.getcwd(), 'src'))

# Locate OpenCV Cascades
cascade_fn = 'haarcascade_frontalface_default.xml'
cascade_path = os.path.join(cv2.data.haarcascades, cascade_fn)
if not os.path.exists(cascade_path):
    print(f"Warning: Could not find {cascade_fn} in {cv2.data.haarcascades}")
    # Fallback: try to find it via direct path or continue without it (mock mode will activate)
else:
    print(f"Found cascade at {cascade_path}")

block_cipher = None

datas = [
    (qml_dir, 'qml'),  # Include QML files
]

if os.path.exists(cascade_path):
    datas.append((cascade_path, '.')) # Put at root of bundle

a = Analysis(
    [os.path.join(src_dir, 'main.py')],
    pathex=[src_dir],
    binaries=[],
    datas=datas,
    hiddenimports=['sqlite3', 'PySide6.QtQuick', 'PySide6.QtQml'],
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    win_no_prefer_redirects=False,
    win_private_assemblies=False,
    cipher=block_cipher,
    noarchive=False,
)
pyz = PYZ(a.pure, a.zipped_data, cipher=block_cipher)

exe = EXE(
    pyz,
    a.scripts,
    [],
    exclude_binaries=True,
    name='RustGuard',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    console=False, # Set to True if you want to see console output for debugging
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
    icon=None, # Add an icon path here if available, e.g., 'icon.ico'
)
coll = COLLECT(
    exe,
    a.binaries,
    a.zipfiles,
    a.datas,
    strip=False,
    upx=True,
    upx_exclude=[],
    name='RustGuard',
)
