#!/usr/bin/env python3
import os
import sys
import platform
import subprocess
import urllib.request

REPO = "https://github.com/aswin402/offpkg"
INSTALL_DIR = os.path.expanduser("~/.offpkg/bin")
IS_WIN = sys.platform == "win32"
BINARY_NAME = "offpkg.exe" if IS_WIN else "offpkg"
LOCAL_BIN = os.path.join(INSTALL_DIR, BINARY_NAME)


def get_target_asset() -> str:
    system = platform.system().lower()
    machine = platform.machine().lower()

    if system == "linux":
        os_name = "linux"
    elif system == "darwin":
        os_name = "macos"
    elif system == "windows":
        os_name = "windows"
    else:
        raise RuntimeError(f"Unsupported OS: {system}")

    if machine in ("x86_64", "amd64"):
        arch_name = "x86_64"
    elif machine in ("aarch64", "arm64"):
        arch_name = "aarch64"
    else:
        raise RuntimeError(f"Unsupported architecture: {machine}")

    ext = ".exe" if IS_WIN else ""
    return f"offpkg-{os_name}-{arch_name}{ext}"


def ensure_binary() -> str:
    if os.path.isfile(LOCAL_BIN):
        return LOCAL_BIN

    os.makedirs(INSTALL_DIR, exist_ok=True)
    asset = get_target_asset()
    url = f"{REPO}/releases/latest/download/{asset}"
    sys.stderr.write(f"[offpkg] Downloading native binary ({asset})...\n")

    try:
        req = urllib.request.Request(url, headers={"User-Agent": "offpkg-installer"})
        with urllib.request.urlopen(req) as resp, open(LOCAL_BIN, "wb") as f:
            f.write(resp.read())
        if not IS_WIN:
            os.chmod(LOCAL_BIN, 0o755)
        sys.stderr.write(f"[offpkg] Successfully installed to {LOCAL_BIN}\n")
    except Exception as e:
        sys.stderr.write(f"[offpkg] Failed to download pre-built binary: {e}\n")
        sys.stderr.write(f"[offpkg] Please install offpkg using: curl -fsSL {REPO}/raw/main/install.sh | bash\n")
        sys.exit(1)

    return LOCAL_BIN


def main():
    binary = ensure_binary()
    try:
        ret = subprocess.call([binary] + sys.argv[1:])
        sys.exit(ret)
    except KeyboardInterrupt:
        sys.exit(130)


if __name__ == "__main__":
    main()
