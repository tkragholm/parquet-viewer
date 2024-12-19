import os
import sys

from setuptools import find_packages, setup
from setuptools.command.install import install
from wheel.bdist_wheel import bdist_wheel


class BinaryDistWheel(bdist_wheel):
    def finalize_options(self):
        super().finalize_options()
        # Mark this as a platform-specific binary package
        self.root_is_pure = False

    def get_tag(self):
        # Override platform tag based on current system
        python, abi, plat = super().get_tag()
        return "py3", "none", plat

class InstallPlatlib(install):
    def finalize_options(self):
        super().finalize_options()
        # Force installation in platlib (platform-specific) directory
        self.install_lib = self.install_platlib

# Determine the binary name based on the platform
binary_name = 'parquet-viewer.exe' if sys.platform == 'win32' else 'parquet-viewer'
binary_path = os.path.join('bin', binary_name)

# Verify binary exists
if not os.path.exists(binary_path):
    raise FileNotFoundError(f"Binary not found: {binary_path}")

setup(
    cmdclass={
        "bdist_wheel": BinaryDistWheel,
        "install": InstallPlatlib,
    },
    packages=find_packages(where="src"),
    package_dir={"": "src"},
    include_package_data=True,
    data_files=[
        ('bin', [binary_path]),
    ],
    scripts=["postinstall.py"],
)
