#!/usr/bin/env bash
set -e

# ── offpkg installer ─────────────────────────────────────────────────────────

REPO="https://github.com/aswin/offpkg"
BINARY="offpkg"
INSTALL_DIR="$HOME/.offpkg/bin"
CYAN="\033[38;2;0;212;224m"
GREEN="\033[38;2;0;229;160m"
AMBER="\033[38;2;245;166;35m"
CORAL="\033[38;2;255;107;107m"
MUTED="\033[38;2;100;116;139m"
BOLD="\033[1m"
RESET="\033[0m"

print_logo() {
  echo ""
  echo -e "${BOLD}${CYAN}╔═╗╔═╗╔═╗╔═╗╦╔═╔═╗${RESET}"
  echo -e "${BOLD}${CYAN}║ ║╠╣ ╠╣ ╠═╝╠╩╗║ ╦${RESET}"
  echo -e "${BOLD}${CYAN}╚═╝╚  ╚  ╩  ╩ ╩╚═╝${RESET}"
  echo -e "  ${MUTED}offpkg · universal offline package manager${RESET}"
  echo ""
}

step() { echo -e "  ${BOLD}${CYAN}→${RESET}  $1"; }
done_() { echo -e "  ${BOLD}${GREEN}✓${RESET}  $1"; }
warn() { echo -e "  ${AMBER}!${RESET}  $1"; }
fail() { echo -e "  ${CORAL}✗${RESET}  $1"; exit 1; }

print_logo

# ── detect OS + arch ─────────────────────────────────────────────────────────

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)  OS_NAME="linux" ;;
  Darwin) OS_NAME="macos" ;;
  MINGW*|MSYS*|CYGWIN*) OS_NAME="windows" ;;
  *) fail "Unsupported OS: $OS" ;;
esac

case "$ARCH" in
  x86_64|amd64) ARCH_NAME="x86_64" ;;
  aarch64|arm64) ARCH_NAME="aarch64" ;;
  *) fail "Unsupported architecture: $ARCH" ;;
esac

step "detected: $OS_NAME / $ARCH_NAME"

# ── check if Rust is available (needed if building from source) ───────────────

has_cmd() { command -v "$1" &>/dev/null; }

# ── install method ────────────────────────────────────────────────────────────
# Priority:
#   1. Pre-built binary from GitHub releases  (fastest)
#   2. Build from source via cargo            (fallback)

RELEASE_URL="${REPO}/releases/latest/download/${BINARY}-${OS_NAME}-${ARCH_NAME}"
if [ "$OS_NAME" = "windows" ]; then
  RELEASE_URL="${RELEASE_URL}.exe"
fi

install_from_release() {
  step "downloading pre-built binary..."

  mkdir -p "$INSTALL_DIR"
  TMP="$(mktemp)"

  if has_cmd curl; then
    HTTP_CODE=$(curl -sL -o "$TMP" -w "%{http_code}" "$RELEASE_URL")
  elif has_cmd wget; then
    wget -qO "$TMP" "$RELEASE_URL"
    HTTP_CODE=200
  else
    return 1
  fi

  if [ "$HTTP_CODE" != "200" ] || [ ! -s "$TMP" ]; then
    rm -f "$TMP"
    return 1
  fi

  chmod +x "$TMP"
  mv "$TMP" "$INSTALL_DIR/$BINARY"
  done_ "binary installed to $INSTALL_DIR/$BINARY"
  return 0
}

install_from_source() {
  if ! has_cmd cargo; then
    echo ""
    warn "Rust not found. Installing Rust first..."
    echo ""
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
    source "$HOME/.cargo/env"
  fi

  if ! has_cmd git; then
    fail "git is required to build from source. Please install git first."
  fi

  step "cloning offpkg repository..."
  TMP_DIR="$(mktemp -d)"
  git clone --depth 1 "$REPO" "$TMP_DIR/offpkg" 2>/dev/null
  cd "$TMP_DIR/offpkg"

  step "building offpkg (this takes ~30 seconds)..."
  cargo build --release --quiet

  mkdir -p "$INSTALL_DIR"
  cp "target/release/$BINARY" "$INSTALL_DIR/$BINARY"
  chmod +x "$INSTALL_DIR/$BINARY"
  cd - >/dev/null
  rm -rf "$TMP_DIR"

  done_ "built and installed to $INSTALL_DIR/$BINARY"
}

# Try pre-built first, fall back to source
if ! install_from_release; then
  warn "no pre-built binary found for $OS_NAME/$ARCH_NAME — building from source"
  echo ""
  install_from_source
fi

# ── add to PATH ───────────────────────────────────────────────────────────────

SHELL_NAME="$(basename "$SHELL")"
add_to_path() {
  local CONFIG_FILE="$1"
  local EXPORT_LINE='export PATH="$HOME/.offpkg/bin:$PATH"'

  if [ -f "$CONFIG_FILE" ] && grep -q ".offpkg/bin" "$CONFIG_FILE" 2>/dev/null; then
    return 0  # already added
  fi

  echo "" >> "$CONFIG_FILE"
  echo "# offpkg" >> "$CONFIG_FILE"
  echo "$EXPORT_LINE" >> "$CONFIG_FILE"
  done_ "added to PATH in $CONFIG_FILE"
}

case "$SHELL_NAME" in
  zsh)  add_to_path "$HOME/.zshrc" ;;
  bash)
    if [ "$OS_NAME" = "macos" ]; then
      add_to_path "$HOME/.bash_profile"
    else
      add_to_path "$HOME/.bashrc"
    fi ;;
  fish) 
    FISH_CONFIG="$HOME/.config/fish/config.fish"
    mkdir -p "$(dirname "$FISH_CONFIG")"
    if ! grep -q ".offpkg/bin" "$FISH_CONFIG" 2>/dev/null; then
      echo "" >> "$FISH_CONFIG"
      echo "# offpkg" >> "$FISH_CONFIG"
      echo 'fish_add_path "$HOME/.offpkg/bin"' >> "$FISH_CONFIG"
      done_ "added to PATH in $FISH_CONFIG"
    fi ;;
  *)
    warn "unknown shell: $SHELL_NAME"
    warn "add this to your shell config manually:"
    echo '    export PATH="$HOME/.offpkg/bin:$PATH"'
    ;;
esac

# ── verify install ────────────────────────────────────────────────────────────

export PATH="$HOME/.offpkg/bin:$PATH"

if "$INSTALL_DIR/$BINARY" --version &>/dev/null; then
  VERSION="$("$INSTALL_DIR/$BINARY" --version)"
  echo ""
  done_ "offpkg $VERSION installed successfully"
else
  warn "binary installed but could not verify — try opening a new terminal"
fi

# ── done ─────────────────────────────────────────────────────────────────────

echo ""
echo -e "  ${MUTED}restart your terminal or run:${RESET}"
echo -e "  ${CYAN}source ~/.zshrc${RESET}  ${MUTED}(or ~/.bashrc)${RESET}"
echo ""
echo -e "  ${MUTED}then get started:${RESET}"
echo -e "  ${CYAN}offpkg doctor${RESET}"
echo -e "  ${CYAN}offpkg stack list${RESET}"
echo ""