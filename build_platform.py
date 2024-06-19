# Creates a build on a specific platform.

import os
import sys
import shutil

print("Building application")

os.system("cargo build --release")
os.system("cargo build --release --package updater")

platform = ""

if sys.platform == "win32":
    platform = "win"
elif sys.platform == "linux":
    platform = "linux"

os.mkdir("temp")

print("Copying files to temp directory")

shutil.copytree("assets", "temp/assets")
shutil.copy("CONTRIBUTING.md", "temp/CONTRIBUTING.md")
shutil.copy("LICENSE", "temp/LICENSE")
shutil.copy("README.md", "temp/README.md")
shutil.copy("logging_config.yaml", "temp/logging_config.yaml")
shutil.copy("VERSION", "temp/VERSION")
shutil.copy("update_log.md", "temp/update_log.md")
shutil.copy(".gitignore", "temp/.gitignore")
shutil.copy("target/release/wavey", "temp/wavey")
shutil.copy("target/release/updater", "temp/updater")

print("Compressing files")

os.system(f"7z a update-package-{platform} temp/*")

print("Cleaning up")

shutil.rmtree("temp")

print("Done")