# Changelog

All notable changes to the **offpkg** project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

---

## [0.1.7] - 2026-10-05

### Added
- **Automated Self-Update Engine**: Integrated version check comparing the local binary against the latest GitHub repository version (`offpkg update self`, `offpkg update --self`, `offpkg self-update`, `offpkg upgrade`).
- **One-Line Curl Installer & Robust Updater**: Upgraded `install.sh` and `update_offpkg.sh` to target official repository `aswin402/offpkg`, verify upstream tags, download prebuilt binaries, or build from source seamlessly.
- **Automated Test Foundation**: Implemented comprehensive unit test suites across all core subsystems:
  - CLI argument parsing tests for Bun, uv, Flutter, Stack, Docs, and global commands with argument validation.
  - SQLite database tests verifying schema initialization, CRUD operations, unique constraints, and filtering.
  - Cache storage tests verifying path resolution, scoped package name flattening (`@scope/pkg` -> `__scope__pkg`), and SHA-256 verification.
  - Manifest parser tests for Dart (`pubspec.yaml`), Python (`pyproject.toml`), and Node.js (`package.json`).
- **Discrete Template Assets**: Extracted large inlined binary and vector assets into static files under `src/stacks/builtin/assets/` (`favicon.svg`, `icons.svg`, `vite.svg`, `react.svg`, `next_favicon.ico`, `hero.png`), embedding them via `include_bytes!`.

### Fixed
- **Cargo Workspace Conflict**: Added standalone `[workspace]` configuration to `Cargo.toml` and excluded `offpkg` from the parent workspace, allowing `cargo check`, `cargo test`, and `cargo build` to run cleanly without parent boundary errors.
- **Dynamic Version Synchronization**: Replaced hardcoded version strings across CLI definitions and TUI ASCII banners with `env!("CARGO_PKG_VERSION")`.
- **Flutter Stack Naming**: Aligned `src/stacks/builtin/flutter.rs` so that the default template name is `flutter-riverpod` instead of `flutter-riverpod-my_app`, matching documentation and commands.
- **Self-Update Repository URL**: Corrected placeholder repository references in `install.sh`, `update_offpkg.sh`, and `src/update.rs` to point to `aswin402/offpkg`.
- **Pubspec Memory Leak**: Eliminated `Box::leak` in `src/adapters/flutter.rs` when inserting dependency lines into `pubspec.yaml`.
- **Source Bloat & Compilation Time**: Reduced `src/stacks/builtin/react_vite.rs` by ~1 MB and drastically sped up build/check cycles from over 60 seconds down to ~1-2 seconds.
- **Formatting & Lint Hygiene**: Resolved internal `rustfmt` parser failures caused by trailing whitespace in `src/stacks/builtin/pern.rs` and `mern.rs`, enabling full project formatting, and eliminated all compiler and Clippy lints across the codebase.

---

## [0.1.6] - 2026-06-06

### Added
- **Kinetic Motion Template (`react-vite-gsap`)**: Built-in stack featuring GSAP, Framer Motion, Lenis Smooth Scroll, shadcn/ui, Lordicon interactive icons, and Lottie React animations.
- **Visual Assets**: Designed official animated SVG logo and documentation assets.

### Optimized
- **SQLite Catalog Queries**: Resolved N+1 query patterns during catalog verification and package existence checks by utilizing batch cache status lookups.

### Documentation
- Updated `README.md` with system benchmarks (RAM usage, binary footprint, sub-15ms CLI dispatch latency).

---

## [0.1.5] - 2026-04-01

### Added
- **Package Updating Engine (`offpkg update`)**: Added ability to check upstream package registries (npm, PyPI, pub.dev) for newer versions and update cached tarballs while preserving user documentation edits.
- **Python Project Ingestion (`offpkg uv install-all`)**: Added automated scanning and downloading of all dependencies declared in `pyproject.toml`.
- **Extended Fullstack Stacks**: Added `fastapi`, `hono-api`, `hono-full`, `mern`, `pern`, and `next-template` built-in stack architectures.
- **Custom Stack Creator**: Added interactive terminal wizard (`offpkg stack new`) to scaffold custom reusable TOML stack templates.

---

## [0.1.4] - 2026-03-27

### Added
- **Flutter Ecosystem Support**: Added `flutter-riverpod` built-in stack with Riverpod 2.0, Hooks, GoRouter, Dio, and Material 3 design tokens.
- **Flutter Bulk Ingestion (`offpkg flutter install-all`)**: Automated parsing of `pubspec.yaml` to download and cache project dependencies.
- **Offline Pub Cache Integration**: Direct archive extraction to `~/.pub-cache/hosted/pub.dev/` with offline resolution fallback.
- **Developer Workflow Tooling**: Added `justfile` with standard commands for check, build, and release.

---

## [0.1.3] - 2026-03-26

### Added
- **Global Documentation Engine (`offpkg docs`)**: Automatic retrieval of package READMEs from package registries during installation.
- **Interactive Documentation Editing**: Added `offpkg docs edit <pkg> --runtime <rt>` to open global Markdown notes in `$EDITOR`.
- **Project Documentation Sync**: Automated copying of customized package guides to `offpkg_docs/offpkg_<pkg>.md` upon adding packages.
- **Documentation Utility Commands**: Added `offpkg docs show`, `offpkg docs reset`, and `offpkg docs list`.

---

## [0.1.2] - 2026-03-25

### Added
- **React + Vite Built-in Stacks**: Added `react-vite` and `react-vite-full` stacks with automated file scaffolding (`vite.config.ts`, `tsconfig.json`, `index.html`, `src/main.tsx`).
- **Binary Asset Ingestion**: Embedded binary assets (`hero.png`) and vector icons for template scaffolding.

---

## [0.1.1] - 2026-03-24

### Added
- **Initial Release**: Core universal offline package manager written in pure Rust.
- **Multi-Runtime Packaging**: Unified interface for Bun (npm), Python (uv/PyPI), and Flutter (pub.dev).
- **Cryptographic Cache Integrity**: SHA-256 verification on all downloaded archives (`.tgz`, `.whl`, `.tar.gz`) stored in `~/.offpkg/cache/`.
- **SQLite Manifest Catalog**: Embedded `offpkg.db` with automated table creation, PRAGMA integrity verification, and package queries.
- **Custom Terminal UI**: Zero-external-dependency ANSI terminal UI with animated braille spinners and progress bars on dedicated background threads.
- **Diagnostic Tool (`offpkg doctor`)**: System environment checks verifying availability of `bun`, `uv`, `flutter`, cache directory status, and database integrity.
- **Cache Management**: Added `remove` command for disk space reclamation and freed-space reporting.
- **Automated Installers**: Added shell installation (`install.sh`) and self-updater (`update_offpkg.sh`) scripts.
