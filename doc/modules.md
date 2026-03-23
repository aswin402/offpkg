# Modules Guide

## src/main.rs
Entry point. Initializes Config, Database, Cache, DocsStore, StackStore, TUI.
Dispatches all CLI commands via `match cli.command { ... }`.

**To add a new command:** add a variant to `Command` in `cli.rs`, add a match arm here.

---

## src/cli.rs
All clap command definitions using `#[derive(Parser, Subcommand)]`.

Enums: `Command`, `BunSubcommand`, `UvSubcommand`, `FlutterSubcommand`, `StackSubcommand`, `DocsSubcommand`

**To add a subcommand:** add a variant to the relevant enum, add fields with `#[arg(...)]`.

---

## src/config.rs
Loads and saves `~/.offpkg/config.toml`. Handles `OFFPKG_CACHE_DIR` env override.

Key APIs:
- `Config::load()` — load or create default config
- `Config::save()` — write to disk
- `Config::cache_path()` — returns resolved cache directory PathBuf

---

## src/db.rs
SQLite via rusqlite. Auto-creates tables on first open. Runs integrity check.

Key APIs:
- `Database::open(config)` — open/create DB
- `insert_package(pkg)` — add to cache manifest
- `list_packages(runtime)` — query all or filtered
- `find_packages(name, runtime)` — find all versions of a package
- `delete_package_version(name, version, runtime)` — remove record
- `count_packages()` — total count

---

## src/cache.rs
File system operations for the package store.

Key APIs:
- `Cache::path_for(runtime, name, version)` — builds cache file path (flattens scoped names)
- `Cache::download_to(url, path)` — async HTTP download
- `Cache::verify_checksum(path, expected)` — SHA-256 verify
- `Cache::ensure_dir(runtime)` — create cache subdirectory
- `compute_sha256(path)` — standalone checksum helper

**Scoped packages:** `@scope/name` → `__scope__name` to avoid path issues.

---

## src/tui.rs
Terminal UI using ANSI escape codes. No ratatui dependency.

Key APIs:
- `TUI::print_line(label, msg, secondary)` — instant colored output
- `TUI::spinner(msg) → Spinner` — animated braille spinner (background thread)
- `TUI::progress_bar(label) → ProgressBar` — animated line fill (background thread)
- `TUI::print_done_summary(pkgs, bytes, elapsed)` — final `✓` summary line
- `TUI::render_logo()` — print offpkg ASCII logo

Labels: `Resolve` (cyan), `Cache` (purple), `Link` (green), `Install` (green),
`Done` (green bold), `Warn` (amber), `Error` (coral), `Info` (blue)

---

## src/doctor.rs
Health checks for all runtimes, cache directory, and DB integrity.

- Runs `bun --version`, `uv --version`, `flutter --version`
- Captures stderr fallback for flutter (outputs version to stderr)
- Shows `✓` / `!` / `✗` per check

**To add a check:** add a `check_runtime(tui, name, &["cmd", "--flag"])` call.

---

## src/remove.rs
Cache pruning. Deletes files from disk and records from DB.

`remove_from_cache(tui, db, pkg, runtime)`:
- Finds all cached versions via `db.find_packages()`
- Deletes each file, accumulates freed bytes
- Removes DB records
- Never touches the project (node_modules / .venv / pubspec)

---

## src/docs.rs
Global docs system. Fetches READMEs from registries, stores in `~/.offpkg/docs/`.

Key APIs:
- `DocsStore::save_docs(runtime, pkg, content)` — save/overwrite global doc
- `DocsStore::copy_to_project(runtime, pkg, dir)` → copies to `offpkg_docs/offpkg_<pkg>.md`
- `DocsStore::open_in_editor(runtime, pkg)` — opens global doc in `$EDITOR`
- `DocsStore::reset_global_doc(runtime, pkg, content)` — overwrite with fresh fetch
- `fetch_docs(runtime, pkg, version)` — async fetch from registry (npm/PyPI/pub.dev)

**Scoped packages:** same flattening as cache — `@vitejs/plugin-react` → `__vitejs__plugin-react.md`

---

## src/stacks.rs
Stack definitions and management.

Built-in stacks: `react-vite`, `react-vite-full`, `hono-api`, `fastapi`, `flutter-riverpod`

Key APIs:
- `builtin_stacks()` — returns all built-in Stack definitions
- `StackStore::all_stacks()` — built-in + custom from `~/.offpkg/stacks/*.toml`
- `StackStore::find(name)` — look up a stack by name
- `StackStore::write_files(stack, dir)` — write config files to project (never overwrites)
- `StackStore::create_template(name, runtime)` — create custom stack TOML

**Stack struct:**
```rust
struct Stack {
    name: String,
    runtime: String,
    description: String,
    packages: Vec<String>,
    dev_packages: Vec<String>,
    files: Vec<StackFile>,  // { path, content }
}
```

---

## src/adapters/bun.rs
Bun/npm adapter.

- `install(pkg)` — resolve from `registry.npmjs.org/<pkg>/latest`, download, cache, fetch docs
- `add(pkg)` — lookup DB, verify checksum, extract tgz to `node_modules/`, update `package.json`, copy docs

---

## src/adapters/uv.rs
Python uv adapter.

- `install(pkg)` — resolve from `pypi.org/pypi/<pkg>/json`, download `.whl`, cache, fetch docs
- `add(pkg)` — lookup DB, verify, run `uv add --frozen --no-index --find-links <cache_dir> <pkg>`, copy docs
- `install_all()` — parse `pyproject.toml`, cache all missing deps

---

## src/adapters/flutter.rs
Flutter/Dart adapter.

- `install(pkg)` — resolve from `pub.dev/api/packages/<pkg>`, download `.tar.gz`, extract to `~/.pub-cache`, cache, fetch docs
- `add(pkg)` — lookup DB, ensure in pub cache, run `flutter pub add` + `flutter pub get --offline`, copy docs
- `install_all()` — parse `pubspec.yaml`, cache all missing deps