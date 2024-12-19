import os
import sys
from pathlib import Path


def install_binary():
    # Get the installation directory
    if sys.platform == 'win32':
        scripts_dir = Path(sys.prefix) / 'Scripts'
    else:
        scripts_dir = Path(sys.prefix) / 'bin'

    # Ensure directory exists
    scripts_dir.mkdir(parents=True, exist_ok=True)

    # Create pv alias/link
    binary_path = scripts_dir / 'parquet-viewer'
    alias_path = scripts_dir / ('pv.exe' if sys.platform == 'win32' else 'pv')

    if sys.platform == 'win32':
        if not alias_path.exists():
            os.link(binary_path, alias_path)
    else:
        if not alias_path.exists():
            os.symlink(binary_path, alias_path)

if __name__ == '__main__':
    install_binary()
