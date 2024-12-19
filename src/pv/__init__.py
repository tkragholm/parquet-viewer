import os
import sys
from pathlib import Path

import pkg_resources


def main():
    try:
        # Get the binary name based on platform
        binary_name = 'parquet-viewer.exe' if sys.platform == 'win32' else 'parquet-viewer'

        # Try to find the binary using pkg_resources
        try:
            binary_path = pkg_resources.resource_filename('pv', os.path.join('bin', binary_name))
        except Exception:
            # Fallback to looking in the package directory
            package_dir = Path(__file__).parent.parent
            binary_path = str(package_dir / 'bin' / binary_name)

        if not os.path.exists(binary_path):
            print(f"Error: Binary not found at {binary_path}", file=sys.stderr)
            sys.exit(1)

        # Make binary executable on Unix-like systems
        if sys.platform != 'win32':
            os.chmod(binary_path, 0o755)

        # Convert all arguments to strings
        args = [str(binary_path)] + [str(arg) for arg in sys.argv[1:]]

        # Execute the binary with any provided arguments
        os.execv(str(binary_path), args)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == '__main__':
    main()
