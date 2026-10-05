# offpkg 🛠️ Universal Offline Package Manager

<p align="center">
  <img src="doc/logo.svg" alt="offpkg Logo" width="220px"/>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.80%2B-blue?logo=rust" alt="Rust"/></a>
  <img src="https://img.shields.io/badge/version-0.1.7-green" alt="Version"/>
  <img src="https://img.shields.io/badge/license-MIT-blue" alt="License"/>
</p>

**offpkg** is a high-performance, offline-first package manager for **Bun (Node.js)**, **Python uv**, and **Flutter**. Cache packages once, instantiate them globally instantly—no network overhead, no configuration pollution.

> Vibe coded with ❤️ by Aswin

---

## Lightweight & High-Performance ⚡

`offpkg` is engineered in pure Rust for minimal overhead, extreme speed, and a tiny system footprint.

| Resource / Metric | Benchmark Value | Details |
| :--- | :--- | :--- |
| **ROM Footprint (Binary)** | **~8.3 MB** | Single fully-compiled, stripped static binary. No interpreter or runtime required to run. |
| **Native RAM Usage** | **~7.3 MB** | Peak resident memory during SQL registry queries & configuration parsing. |
| **CLI Dispatch Latency** | **< 15 ms** | Sub-millisecond command parsing using `clap` and quick SQLite catalog lookups. |
| **Offline Link Speed** | **Instant (Milliseconds)** | Local file hardlinking and `.tgz` unpacking directly to project roots. |

---

## Core Features ✨

- **📦 Multi-Runtime Cache Wrapper**: Unified interface for Bun (npm), Python (uv/PyPI), and Flutter (pub.dev).
- **🔒 Secure Local Archiving**: Downloads and verifies tarballs and wheels via SHA-256 hashes, stored in `~/.offpkg/cache/`.
- **🗃️ SQLite Database Manifest**: Blazing-fast cataloging in `offpkg.db` with auto-repair and integrity diagnostics.
- **🏗️ Pristine Stacks**: Scaffold entire application templates offline in under a second (configurations, files, and deep transitive dependencies).
- **📝 Global Offline Docs**: Modify fetched package READMEs once with `offpkg docs edit`, and have your custom versions auto-copied to every new project folder under `offpkg_docs/`.
- **🩺 Interactive TUI & Doctor**: Features animated spinners, colored status labels, progress bars, and environment diagnostics to verify runtime compatibility.
- **🔄 Auto-Updating Engine**: Update packages or offpkg itself seamlessly via `offpkg update self` or `offpkg self-update`.
- **🧹 Cache Pruner**: `remove <pkg>` deletes cached archives and DB entries while displaying freed disk space.

---

## How It Works 🔄

```
Online once:                          Offline forever:
─────────────────────────────         ──────────────────────────────────
offpkg bun install react         →    offpkg bun add react      (project A)
offpkg stack install react-vite  →    offpkg stack add react-vite (project B)
offpkg docs edit react           →    offpkg_docs/offpkg_react.md (every project)
```

---

## CLI Logo

```
╔═╗╔═╗╔═╗╔═╗╦╔═╔═╗
║ ║╠╣ ╠╣ ╠═╝╠╩╗║ ╦
╚═╝╚  ╚  ╩  ╩ ╩╚═╝
offpkg v0.1.7 · universal offline package manager
```

---

## Quickstart 🚀

### Option 1: One-Line Install (Recommended)

Run the automated installer script via `curl` or `wget`. It auto-detects your operating system and architecture, downloads the pre-built binary (or compiles from source if pre-built is unavailable), installs it to `~/.offpkg/bin`, and configures your shell `$PATH`:

```bash
# Using curl (Linux & macOS)
curl -fsSL https://raw.githubusercontent.com/aswin402/offpkg/main/install.sh | bash

# Or using wget
wget -qO- https://raw.githubusercontent.com/aswin402/offpkg/main/install.sh | bash
```

After installation, reload your shell:
```bash
source ~/.bashrc   # or ~/.zshrc / source ~/.config/fish/config.fish
```

---

### Option 2: Direct Binary Download (GitHub Releases)

Download pre-compiled, self-contained static binaries directly from [GitHub Releases](https://github.com/aswin402/offpkg/releases/latest):

| Platform | Architecture | Binary Asset |
|---|---|---|
| **Linux** | `x86_64` (Intel/AMD) | [`offpkg-linux-x86_64`](https://github.com/aswin402/offpkg/releases/latest/download/offpkg-linux-x86_64) |
| **Linux** | `aarch64` (ARM64) | [`offpkg-linux-aarch64`](https://github.com/aswin402/offpkg/releases/latest/download/offpkg-linux-aarch64) |
| **macOS** | `Apple Silicon` (M1/M2/M3/M4) | [`offpkg-macos-aarch64`](https://github.com/aswin402/offpkg/releases/latest/download/offpkg-macos-aarch64) |
| **macOS** | `x86_64` (Intel) | [`offpkg-macos-x86_64`](https://github.com/aswin402/offpkg/releases/latest/download/offpkg-macos-x86_64) |
| **Windows** | `x86_64` | [`offpkg-windows-x86_64.exe`](https://github.com/aswin402/offpkg/releases/latest/download/offpkg-windows-x86_64.exe) |

#### Quick Manual Setup (Linux / macOS):
```bash
# 1. Download the binary for your platform
curl -sL -o offpkg https://github.com/aswin402/offpkg/releases/latest/download/offpkg-linux-x86_64

# 2. Make it executable
chmod +x offpkg

# 3. Move to your PATH
sudo mv offpkg /usr/local/bin/
# (or: mkdir -p ~/.offpkg/bin && mv offpkg ~/.offpkg/bin/ && export PATH="$HOME/.offpkg/bin:$PATH")
```

---

### Option 3: Install via Cargo (Rust)

If you have Rust installed, install directly from the GitHub repository:

```bash
cargo install --git https://github.com/aswin402/offpkg.git
```

---

### Self-Updating 🔄

Keep `offpkg` up to date with the latest templates and improvements using a single command:

```bash
# Check GitHub releases and update binary automatically
offpkg update self

# Or using the self-update alias
offpkg self-update
```

If your installation is already on the latest version, `offpkg` immediately confirms:
```
[ info    ]  checking for offpkg updates...
[ done    ]  offpkg is already up to date (v0.1.7)
```

---

### Verification & Health Check 🩺

Verify the installation and check runtime environment compatibility:

```bash
# Check version banner
offpkg --version

# Run diagnostic suite for bun, uv, flutter, SQLite & cache
offpkg doctor

# List all available built-in project stacks
offpkg stack list
```

---

## CLI Reference

### Per-runtime commands

```bash
# Download and cache a package globally (needs internet, run once)
offpkg bun install <pkg>
offpkg uv install <pkg>
offpkg flutter install <pkg>

# Add a cached package to the current project (fully offline)
offpkg bun add <pkg>
offpkg uv add <pkg>
offpkg flutter add <pkg>

# Cache all deps from existing project file (needs internet)
offpkg uv install-all        # reads pyproject.toml
offpkg flutter install-all   # reads pubspec.yaml

# Remove a package from the offpkg cache
offpkg bun remove <pkg>
offpkg uv remove <pkg>
offpkg flutter remove <pkg>
```

### Stack commands

```bash
# Cache all packages in a stack globally (needs internet, run once)
offpkg stack install react-vite

# Add a full stack to the current project (fully offline)
offpkg stack add react-vite

# List all available stacks
offpkg stack list

# Show what a stack contains
offpkg stack show react-vite

# Create a custom stack template
offpkg stack new my-stack --runtime bun
```

### Docs commands

```bash
# Edit global doc in $EDITOR — your edits apply to all future project copies
offpkg docs edit <pkg> --runtime bun

# Print global doc to terminal
offpkg docs show <pkg> --runtime bun

# Regenerate from original registry README
offpkg docs reset <pkg> --runtime bun

# List all packages with cached docs
offpkg docs list
offpkg docs list --runtime flutter
```

### Global commands

```bash
offpkg list                        # show all cached packages
offpkg list --runtime bun          # filter by runtime
offpkg doctor                      # environment health check
offpkg update self                 # check for new version and update offpkg binary
offpkg self-update                 # alias for binary update
```

---

## Built-in Stacks

| Stack | Runtime | Description / Packages |
|---|---|---|
| **`react-vite`** | bun | React 19 + Vite 8 + Tailwind 4 + Zustand + TanStack Query + React Router 7 |
| **`react-vite-full`** | bun | Complete Modern Template (above + zod, axios, hook-form, etc.) |
| **`react-vite-gsap`** | bun | GSAP + Framer Motion + Lenis Smooth Scroll + shadcn/ui + Lordicon (Kinetic Motion Template) |
| **`hono-api`** | bun | Hono API + Pino logger |
| **`hono-full`** | bun | Hono + Prisma + Zod + Pino + Better Auth (Prisma Adapter) |
| **`next-template`** | bun | Upgraded Next.js 16 + Tailwind v4 + Prisma 7 + Professional Backend |
| **`mern`** | bun | MERN Stack - Express + React + MongoDB + TypeScript (Monorepo) |
| **`pern`** | bun | PERN Stack - Express + React + PostgreSQL + Prisma + TypeScript (Monorepo) |
| **`fastapi`** | uv | FastAPI + SQLAlchemy (Async) + Alembic + Pydantic v2 + structlog |
| **`flutter-riverpod`** | flutter | Flutter + Riverpod/Hooks + GoRouter + Dio + Material 3 + Logger + Google Fonts |

Each stack also generates starter config files (`vite.config.ts`, `tsconfig.json`, `main.dart`, etc.) automatically.
Furthermore, the React/Next stacks natively embed global **binary assets** (e.g. `hero.png`) and fully resolve **all deep dependencies** directly into `node_modules` without altering your project configuration.

---

## Workflows

### 1. First time setup

```bash
offpkg doctor       # check runtimes
offpkg stack list   # see available stacks
```

### 2. React + Vite project (offline)

```bash
# Online once
offpkg stack install react-vite-full

# Offline forever
mkdir my-app && cd my-app
bun init -y
offpkg stack add react-vite-full
# → installs all packages into node_modules/
# → writes vite.config.ts, tsconfig.json, index.html, src/main.tsx
# → writes src/store.ts (zustand), wraps app in QueryClientProvider
# → copies docs into offpkg_docs/
```

### 3. FastAPI project (offline)

```bash
# Online once
offpkg stack install fastapi

# Offline forever
mkdir my-api && cd my-api
uv init
offpkg stack add fastapi
# → runs uv add --frozen for all packages
# → writes app/main.py, .env, alembic.ini
```

### 4. Flutter project (offline)

```bash
# Online once
offpkg stack install flutter-riverpod

# Offline forever
cd my_flutter_app
offpkg stack add flutter-riverpod
# → extracts to ~/.pub-cache, runs flutter pub get --offline
# → writes lib/main.dart with ProviderScope boilerplate
```

### 5. Global docs workflow

```bash
# install caches the package + fetches README
offpkg bun install react

# edit the global doc — add your own notes, examples, team conventions
offpkg docs edit react --runtime bun
# opens ~/.offpkg/docs/bun/react.md in $EDITOR

# every project you add react to gets YOUR edited version
cd project-a && offpkg bun add react
# → node_modules/react/ installed
# → offpkg_docs/offpkg_react.md copied (your edited version)

cd project-b && offpkg bun add react
# → same edited doc copied here too
```

### 6. Custom stack

```bash
# Create template
offpkg stack new my-fullstack --runtime bun
# → creates ~/.offpkg/stacks/my-fullstack.toml

# Edit the TOML to add your packages and starter files
nano ~/.offpkg/stacks/my-fullstack.toml

# Cache it (needs internet once)
offpkg stack install my-fullstack

# Use it in any project (offline)
offpkg stack add my-fullstack
```

---

## Directory Structure

```
~/.offpkg/
├── config.toml              # global config
├── stacks/                  # your custom stack definitions
│   └── my-stack.toml
└── cache/
    ├── offpkg.db            # sqlite manifest
    ├── bun/                 # npm tarballs (.tgz)
    ├── uv/                  # python wheels (.whl)
    ├── flutter/             # pub archives (.tar.gz)
    └── docs/
        ├── bun/             # editable package docs
        │   └── react.md
        ├── uv/
        │   └── fastapi.md
        └── flutter/
            └── riverpod.md
```

---

## Configuration

```toml
# ~/.offpkg/config.toml

[cache]
path = "~/.offpkg/cache"
max_size_gb = 50.0

[network]
timeout_secs = 30
retries = 3

[runtimes]
bun = "auto"       # auto = detect from PATH
uv = "auto"
flutter = "auto"
```

**Override cache location:**
```bash
OFFPKG_CACHE_DIR=/external/drive offpkg bun install react
```

---

## Doctor Output

```
╔═╗╔═╗╔═╗╔═╗╦╔═╔═╗
║ ║╠╣ ╠╣ ╠═╝╠╩╗║ ╦
╚═╝╚  ╚  ╩  ╩ ╩╚═╝
  offpkg v0.1.3 · universal offline package manager

[ info  ]  offpkg doctor        running environment checks
[ done  ]  bun                  bun 1.3.4
[ done  ]  uv                   uv 0.10.7
[ done  ]  flutter              Flutter 3.22.0 · channel stable
[ done  ]  cache directory      ~/.offpkg/cache — 3 entries
[ done  ]  database             30 package(s) cached — integrity ok
[ done  ]  doctor complete
```

---

## Database Schema

```sql
CREATE TABLE packages (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL,
    version     TEXT NOT NULL,
    runtime     TEXT NOT NULL CHECK(runtime IN ('bun','uv','flutter')),
    cache_path  TEXT NOT NULL,
    checksum    TEXT NOT NULL,
    size_bytes  INTEGER,
    cached_at   DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(name, version, runtime)
);

CREATE TABLE config (
    key   TEXT PRIMARY KEY,
    value TEXT
);
```

---

## Architecture

```
CLI (clap)
    │
    ▼
main.rs ──── Config ──── Database (SQLite)
    │              └──── Cache (~/.offpkg/cache/)
    │
    ├── BunAdapter     → npmjs.org
    ├── UvAdapter      → pypi.org
    ├── FlutterAdapter → pub.dev
    ├── StackStore     → ~/.offpkg/stacks/
    ├── DocsStore      → ~/.offpkg/docs/
    ├── remove         → cache pruning
    └── doctor         → health checks

TUI (ANSI) — spinner · progress bar · labels · summary
```

---

## Modules

| Module | Purpose |
|---|---|
| `main.rs` | CLI dispatch, wires all modules together |
| `cli.rs` | All clap command/subcommand definitions |
| `config.rs` | Load/save `~/.offpkg/config.toml` |
| `db.rs` | SQLite — insert/query/delete packages |
| `cache.rs` | File store — download, checksum, path resolution |
| `tui.rs` | Terminal UI — spinner, progress bar, colored labels |
| `doctor.rs` | Runtime health checks and cache diagnostics |
| `remove.rs` | Cache pruning — delete files and DB records |
| `docs.rs` | Global docs — fetch READMEs, edit, copy to projects |
| `stacks.rs` | Stack definitions, install/add, file generation |
| `adapters/bun.rs` | Bun/npm install and add logic |
| `adapters/uv.rs` | Python uv install and add logic |
| `adapters/flutter.rs` | Flutter pub install and add logic |

---

## Roadmap

- [x] Full CLI — bun/uv/flutter add/install/remove/install-all
- [x] Registry resolution — npm, PyPI, pub.dev
- [x] Animated TUI — spinner, progress bar, colored labels
- [x] Global docs system — fetch, edit, copy to projects
- [x] Stacks — full project setup with one command
- [x] Cache prune with freed space reporting
- [ ] `offpkg update <pkg>` — update cached version
- [ ] Auto-prune by size/age
- [ ] Binary releases (GitHub Actions)
- [ ] Windows support
- [ ] VSCode extension

---

## Contributing

```bash
git clone <repo> && cd offpkg
cargo check        # must be zero warnings
cargo fmt
cargo clippy
cargo build --release
```

Adding a new runtime adapter: copy `src/adapters/bun.rs`, update the registry URL and file format, add the subcommand to `cli.rs`, wire it in `main.rs`, add a check in `doctor.rs`.

PRs welcome — fork, branch `feat/xyz`, update docs.

---

**License**: MIT