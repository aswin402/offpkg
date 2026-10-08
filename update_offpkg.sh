#!/usr/bin/env bash
set -e

# ── offpkg self-updater ───────────────────────────────────────────────────────

INSTALL_DIR="${INSTALL_DIR:-$HOME/.offpkg/bin}"
BINARY="offpkg"
REPO="https://github.com/aswin402/offpkg"
RAW_CARGO="https://raw.githubusercontent.com/aswin402/offpkg/main/Cargo.toml"
CYAN="\033[38;2;0;212;224m"
GREEN="\033[38;2;0;229;160m"
AMBER="\033[38;2;245;166;35m"
CORAL="\033[38;2;255;107;107m"
MUTED="\033[38;2;100;116;139m"
BOLD="\033[1m"
RESET="\033[0m"

step() { echo -e "  ${BOLD}${CYAN}→${RESET}  $1"; }
done_() { echo -e "  ${BOLD}${GREEN}✓${RESET}  $1"; }
warn() { echo -e "  ${AMBER}!${RESET}  $1"; }
fail() { echo -e "  ${CORAL}✗${RESET}  $1"; exit 1; }

echo ""
echo -e "${BOLD}${CYAN}offpkg updater${RESET}"
echo -e "${MUTED}──────────────────${RESET}"
echo ""

# Current version
CURRENT_VERSION=""
if [ -f "$INSTALL_DIR/$BINARY" ]; then
  CURRENT_STR="$("$INSTALL_DIR/$BINARY" --version 2>/dev/null || echo 'unknown')"
  CURRENT_VERSION=$(echo "$CURRENT_STR" | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -n1 || echo '')
  step "current version: ${CURRENT_STR:-$CURRENT_VERSION}"
else
  warn "offpkg not found at $INSTALL_DIR — running fresh install"
  curl -fsSL "${REPO}/raw/main/install.sh" | bash
  exit 0
fi

# Check latest version from repository
step "checking for updates..."
LATEST_VERSION="$(curl -fsSL "$RAW_CARGO" 2>/dev/null | grep -E '^version = ' | head -n1 | cut -d '"' -f2 || echo '')"

if [ -n "$LATEST_VERSION" ] && [ -n "$CURRENT_VERSION" ]; then
  if [ "$CURRENT_VERSION" = "$LATEST_VERSION" ] && [ "$1" != "--force" ] && [ "$1" != "-f" ]; then
    done_ "offpkg is already up to date (v${CURRENT_VERSION})"
    exit 0
  fi
  step "new version available: v${CURRENT_VERSION} → v${LATEST_VERSION}"
fi

# Detect OS/arch
OS="$(uname -s)"
ARCH="$(uname -m)"
case "$OS" in
  Linux)  OS_NAME="linux" ;;
  Darwin) OS_NAME="macos" ;;
  MINGW*|MSYS*|CYGWIN*) OS_NAME="windows" ;;
  *) fail "Unsupported OS: $OS" ;;
esac
case "$ARCH" in
  x86_64|amd64)  ARCH_NAME="x86_64" ;;
  aarch64|arm64) ARCH_NAME="aarch64" ;;
  *) fail "Unsupported architecture: $ARCH" ;;
esac

has_cmd() { command -v "$1" &>/dev/null; }

# ── Try pre-built binary first ────────────────────────────────────────────────

try_prebuilt() {
  step "checking for pre-built binary..."
  TMP="$(mktemp)"

  local EXT=""
  [ "$OS_NAME" = "windows" ] && EXT=".exe"

  local URLS=()
  if [ -n "$LATEST_VERSION" ]; then
    URLS+=("${REPO}/releases/download/v${LATEST_VERSION}/${BINARY}-${OS_NAME}-${ARCH_NAME}${EXT}")
  fi
  URLS+=("${REPO}/releases/latest/download/${BINARY}-${OS_NAME}-${ARCH_NAME}${EXT}")

  for URL in "${URLS[@]}"; do
    local HTTP_CODE=0
    if has_cmd curl; then
      HTTP_CODE=$(curl -sL -o "$TMP" -w "%{http_code}" "$URL")
    elif has_cmd wget; then
      wget -qO "$TMP" "$URL" && HTTP_CODE=200
    fi

    if [ "$HTTP_CODE" = "200" ] && [ -s "$TMP" ]; then
      mkdir -p "$INSTALL_DIR"
      chmod +x "$TMP"
      mv "$TMP" "$INSTALL_DIR/$BINARY"
      return 0
    fi
  done

  rm -f "$TMP"
  return 1
}

# ── Or rebuild from source ────────────────────────────────────────────────────

try_source() {
  if ! has_cmd cargo; then
    fail "Rust/cargo not found. Install from https://rustup.rs"
  fi
  if ! has_cmd git; then
    fail "git is required to build from source"
  fi

  step "pulling latest source..."
  TMP_DIR="$(mktemp -d)"
  git clone --depth 1 "$REPO" "$TMP_DIR/offpkg" 2>/dev/null
  cd "$TMP_DIR/offpkg"

  step "rebuilding offpkg..."
  cargo build --release --quiet

  cp "target/release/$BINARY" "$INSTALL_DIR/$BINARY"
  chmod +x "$INSTALL_DIR/$BINARY"
  cd - >/dev/null
  rm -rf "$TMP_DIR"
}

if ! try_prebuilt; then
  warn "no pre-built binary found — rebuilding from source"
  echo ""
  try_source
fi

# ── Show new version ──────────────────────────────────────────────────────────

export PATH="$HOME/.offpkg/bin:$PATH"
NEW="$("$INSTALL_DIR/$BINARY" --version 2>/dev/null || echo 'unknown')"

echo ""
done_ "updated to $NEW"
echo ""
echo -e "  ${MUTED}your cached packages and docs are untouched${RESET}"
echo ""