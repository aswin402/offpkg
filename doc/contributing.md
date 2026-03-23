# Contributing & Modifications

## Setup

```bash
git clone <repo> && cd offpkg
rustup update stable
cargo check          # must be zero warnings
cargo fmt
cargo clippy
cargo build --release
```

## Code Style

- Zero warnings — `cargo check` must be clean before any PR
- `cargo fmt` for formatting
- `cargo clippy` for lints
- No unused imports, no unused variables

## Adding a New Runtime

1. Create `src/adapters/<runtime>.rs` — copy `bun.rs` as a template
2. Update registry URL, file extension, and package manifest logic
3. Add `pub mod <runtime>;` to the `adapters` module in `main.rs`
4. Add subcommand variants to `cli.rs`
5. Add a match arm in `main.rs`
6. Add `check_runtime(tui, "<runtime>", &["<cmd>", "--version"])` to `doctor.rs`
7. Add `"<runtime>"` to the SQLite CHECK constraint in `db.rs`

## Adding a Built-in Stack

Add a new `Stack { ... }` entry to the `builtin_stacks()` vec in `src/stacks.rs`:

```rust
Stack {
    name: "my-stack".into(),
    runtime: "bun".into(),
    description: "Description here".into(),
    packages: vec!["pkg-a".into(), "pkg-b".into()],
    dev_packages: vec!["typescript".into()],
    files: vec![
        StackFile {
            path: "src/index.ts".into(),
            content: r#"// starter content"#.into(),
        },
    ],
},
```

## Testing Changes

```bash
# Compile check
cargo check

# Full build
cargo build

# Test doctor
cargo run -- doctor

# Test install
cargo run -- bun install lodash

# Test add (from project dir)
cd /tmp/test-project && bun init -y
cargo run --manifest-path ~/offpkg/Cargo.toml -- bun add lodash

# Test stack
cargo run -- stack install react-vite
mkdir /tmp/react-test && cd /tmp/react-test && bun init -y
cargo run --manifest-path ~/offpkg/Cargo.toml -- stack add react-vite

# Test docs
cargo run -- docs edit lodash --runtime bun
cargo run -- docs show lodash --runtime bun
cargo run -- docs list

# Test remove
cargo run -- bun remove lodash
cargo run -- list
```

## Build Release Binary

```bash
cargo build --release
strip target/release/offpkg
ls -lh target/release/offpkg

# Install globally
cargo install --path .
```

## PR Guidelines

- Fork the repo, create branch `feat/<name>` or `fix/<name>`
- One feature/fix per PR
- Update relevant docs in `docs/`
- `cargo check` must pass with zero warnings
- Add a brief description of what changed and why

**License**: MIT