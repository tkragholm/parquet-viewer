import subprocess
import sys
from pathlib import Path


def main():
    try:
        # Get the path to the installed binary
        package_dir = Path(__file__).parent
        binary_name = (
            "parquet-viewer.exe" if sys.platform == "win32" else "parquet-viewer"
        )
        binary_path = package_dir.parent.parent / "bin" / binary_name

        if not binary_path.exists():
            print(f"Error: Binary not found at {binary_path}", file=sys.stderr)
            sys.exit(1)

        # Make binary executable on Unix-like systems
        if sys.platform != "win32":
            binary_path.chmod(binary_path.stat().st_mode | 0o755)

        # Execute the binary with any provided arguments
        result = subprocess.run([str(binary_path)] + sys.argv[1:])
        sys.exit(result.returncode)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
