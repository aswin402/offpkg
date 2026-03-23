# Architecture

## High-Level Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         CLI (clap)                              │
│              Args → Command → Subcommand                        │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                          main.rs                                │
│   init: Config → Database → Cache → DocsStore → StackStore     │
│   dispatch: match cli.command { ... }                           │
└──┬──────────┬──────────┬──────────┬──────────┬─────────────────┘
   │          │          │          │          │
   ▼          ▼          ▼          ▼          ▼
BunAdapter  UvAdapter  Flutter   StackStore  DocsStore
(npm)       (PyPI)     Adapter   ~/.offpkg/  ~/.offpkg/
                       (pub.dev) stacks/     docs/
   │          │          │
   ▼          ▼          ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Cache (~/.offpkg/cache/)                      │
│   bun/*.tgz   uv/*.whl   flutter/*.tar.gz   offpkg.db           │
└─────────────────────────────────────────────────────────────────┘
```

## Data Flow: `offpkg bun install react`

```
1. CLI parse → BunAdapter::install("react")
2. spinner("resolving react from npm registry...")
3. GET https://registry.npmjs.org/react/latest → version, tarball_url
4. spinner.finish → [ resolve ] react@19.2.0  npm registry
5. progress_bar("downloading react@19.2.0")
6. Cache::download_to(tarball_url, ~/.offpkg/cache/bun/react@19.2.0.tgz)
7. compute_sha256(cache_path) → checksum
8. progress_bar.finish
9. fetch_docs("bun", "react", "19.2.0") → README from npm
10. DocsStore::save_docs → ~/.offpkg/docs/bun/react.md
11. Database::insert_package(...)
12. [ install ] react@19.2.0 cached
13. print_done_summary(1, size, elapsed)
```

## Data Flow: `offpkg bun add react` (offline)

```
1. CLI parse → BunAdapter::add("react")
2. spinner("resolving react from offpkg cache...")
3. Database::list_packages(Some("bun")) → filter → latest version
4. Cache::verify_checksum(cache_path, checksum) → ok
5. spinner.finish → [ resolve ] react@19.2.0  found in offpkg cache
6. [ cache ] reading react@19.2.0.tgz
7. progress_bar("extracting & linking react")
8. extract_tgz(cache_path, cwd/node_modules/react/)
9. update_package_json(cwd, "react", "19.2.0")
10. progress_bar.finish
11. DocsStore::copy_to_project → cwd/offpkg_docs/offpkg_react.md
12. [ link ] react → node_modules/react
13. [ done ] 1 package installed  no network used
14. print_done_summary(1, 0, elapsed)
```

## Data Flow: `offpkg stack add react-vite` (offline)

```
1. StackStore::find("react-vite") → Stack { packages, dev_packages, files }
2. For each pkg: BunAdapter::add(pkg)
3. StackStore::write_files → vite.config.ts, tsconfig.json, src/main.tsx ...
4. DocsStore::copy_to_project for each pkg
5. [ done ] stack react-vite ready  7/7 packages · 5 files
```

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

## Scoped Package Handling

npm scoped packages contain `@` and `/` which are invalid in file paths.
offpkg flattens them consistently across cache, docs, and project files:

```
@vitejs/plugin-react  →  __vitejs__plugin-react
@types/react          →  __types__react
```

Applied in: `Cache::path_for()`, `DocsStore::global_doc_path()`, `DocsStore::copy_to_project()`

## TUI Architecture

```
TUI (Clone)
├── print_line(label, msg, secondary)   instant labeled output
├── spinner(msg) → Spinner             background thread, braille frames @80ms
│   └── .finish(label, msg, sec)       stop + print label line
└── progress_bar(label) → ProgressBar  background thread, line fill @16ms
    ├── .set(pct 0.0-1.0, label)       update with easing
    └── .finish(msg)                   stop + print full cyan line

Both implement Drop — auto-cleanup on error/early exit
```