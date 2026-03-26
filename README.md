# offpkg 🛠️ Universal Offline Package Manager

[![Rust](https://img.shields.io/badge/Rust-1.80%2B-blue?logo=rust)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-0.1.1--beta-green)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()

**offpkg** is a high-performance, offline-first package manager for **Bun**, **Python uv**, and **Flutter**. Download packages once, use them anywhere — no internet required.

> vibe coded by Aswin

---

## What's Inside ✨

| Feature | Description |
|---|---|
| **Offline Caching** | Downloads tarballs/wheels to `~/.offpkg/cache/` with SHA-256 verification |
| **Multi-Runtime** | Full support for Bun (npm), Python uv (PyPI), Flutter (pub.dev) |
| **Stacks** | One command to install a full project setup — `offpkg stack add react-vite` |
| **Global Docs** | Edit package READMEs once in `~/.offpkg/docs/`, auto-copied to every project |
| **SQLite DB** | Tracks all cached packages in `offpkg.db` with checksums and metadata |
| **Pristine Templates** | Deep transitive dependency resolution strictly maps into `node_modules/` without polluting `package.json` |
| **Binary Assets** | Stack templates natively embed `.png` and binary core files directly inside the Rust CLI |
| **Beautiful TUI** | Animated spinner, progress bar, colored labels — inspired by Bun's output |
| **Doctor** | Runtime health checks, cache integrity, DB diagnostics |
| **Cache Prune** | `remove <pkg>` deletes cache + DB record, shows freed space |

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

## Logo

```
╔═╗╔═╗╔═╗╔═╗╦╔═╔═╗
║ ║╠╣ ╠╣ ╠═╝╠╩╗║ ╦
╚═╝╚  ╚  ╩  ╩ ╩╚═╝
offpkg v0.1.3 · universal offline package manager
```

---

## Quickstart 🚀

### Prerequisites

- Rust 1.80+
- At least one runtime: `bun`, `uv`, `flutter` (all optional — doctor shows what's missing)

### Build & Install

```bash
git clone <repo> && cd offpkg
cargo build --release
cargo install --path .

# Verify
offpkg doctor
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
```

---

## Built-in Stacks

| Stack | Runtime | Packages |
|---|---|---|
| *`react-vite`* | bun | react, react-dom, vite, typescript, etc. + **210+ transitive dependencies** |
| *`react-vite-full`* | bun | above + zustand, @tanstack/react-query, hook-form, zod, axios + **210+ transitive** |
| `hono-api` | bun | hono, @prisma/client, zod, pino, pg, dotenv + dev: typescript, prisma, pino-pretty |
| `fastapi` | uv | fastapi, uvicorn, sqlalchemy, asyncpg, alembic, pydantic-settings, python-dotenv, structlog |
| `flutter-riverpod` | flutter | flutter_riverpod, hooks_riverpod, flutter_hooks, go_router, dio, logger |

Each stack also generates starter config files (`vite.config.ts`, `tsconfig.json`, `main.dart`, etc.) automatically.
Furthermore, the React-Vite stacks natively embed global **binary assets** (e.g. `hero.png`) and fully resolve **all deep dependencies** directly into `node_modules` without altering your project configuration.

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