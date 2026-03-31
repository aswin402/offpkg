use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "offpkg",
    version = "0.1.5",
    about = "Universal offline package manager — cache packages once, install forever",
    long_about = "offpkg caches packages from npm (bun), PyPI (uv), and pub.dev (flutter)\nso you can install them later without an internet connection.\n\nWorkflow:\n  1. Cache packages online  →  offpkg <runtime> install <pkg>\n  2. Add to project offline →  offpkg <runtime> add <pkg>",
    disable_version_flag = true
)]
pub struct Args {
    /// Print version information
    #[arg(short = 'V', long = "version")]
    pub version: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Bun (npm) package commands — cache and install JavaScript packages
    Bun {
        #[command(subcommand)]
        subcmd: BunSubcommand,
    },
    /// uv (PyPI) package commands — cache and install Python packages
    Uv {
        #[command(subcommand)]
        subcmd: UvSubcommand,
    },
    /// Flutter (pub.dev) package commands — cache and install Dart/Flutter packages
    Flutter {
        #[command(subcommand)]
        subcmd: FlutterSubcommand,
    },
    /// Stack commands — install full project setups with one command
    Stack {
        #[command(subcommand)]
        subcmd: StackSubcommand,
    },
    /// Update cached packages to latest versions (docs are never changed)
    Update {
        /// Specific package name to update (omit to update all)
        pkg: Option<String>,
        /// Only update packages for this runtime
        #[arg(long, value_parser = ["bun", "uv", "flutter"])]
        runtime: Option<String>,
    },
    /// List all cached packages (optionally filter by runtime)
    List {
        /// Filter by runtime: bun, uv, or flutter
        #[arg(long, value_parser = ["bun", "uv", "flutter"])]
        runtime: Option<String>,
    },
    /// Package documentation commands — view, edit, or reset cached docs
    Docs {
        #[command(subcommand)]
        subcmd: DocsSubcommand,
    },
    /// Run diagnostics to check offpkg health and configuration
    Doctor,
    /// Update offpkg itself to the latest version
    SelfUpdate,
}

// ── Bun ──────────────────────────────────────────────────────────────────────

#[derive(Subcommand, Clone, Debug)]
pub enum BunSubcommand {
    /// Cache a bun package from npm (requires internet, run once)
    Install {
        /// Package name to download and cache (e.g. react, lodash)
        pkg: String,
    },
    /// Add a cached bun package to the current project (works offline)
    Add {
        /// Package name to add (must already be cached via 'install')
        pkg: String,
    },
    /// Remove a bun package from the local cache
    Remove {
        /// Package name to remove from cache
        pkg: String,
    },
    /// List all cached bun packages
    List,
    /// Update cached bun packages to latest versions (docs unchanged)
    Update {
        /// Package name to update (omit to update all cached bun packages)
        pkg: Option<String>,
    },
}

// ── uv ───────────────────────────────────────────────────────────────────────

#[derive(Subcommand, Clone, Debug)]
pub enum UvSubcommand {
    /// Cache a uv/PyPI package (requires internet, run once)
    Install {
        /// Package name to download and cache (e.g. requests, numpy)
        pkg: String,
    },
    /// Add a cached uv package to the current project (works offline)
    Add {
        /// Package name to add (must already be cached via 'install')
        pkg: String,
    },
    /// Add all cached uv packages to the current project (works offline)
    InstallAll,
    /// Remove a uv package from the local cache
    Remove {
        /// Package name to remove from cache
        pkg: String,
    },
    /// List all cached uv packages
    List,
    /// Update cached uv packages to latest versions (docs unchanged)
    Update {
        /// Package name to update (omit to update all cached uv packages)
        pkg: Option<String>,
    },
}

// ── Flutter ───────────────────────────────────────────────────────────────────

#[derive(Subcommand, Clone, Debug)]
pub enum FlutterSubcommand {
    /// Cache a Flutter/pub.dev package (requires internet, run once)
    Install {
        /// Package name to download and cache (e.g. provider, dio)
        pkg: String,
    },
    /// Add a cached Flutter package to the current project (works offline)
    Add {
        /// Package name to add (must already be cached via 'install')
        pkg: String,
    },
    /// Add all cached Flutter packages to the current project (works offline)
    InstallAll,
    /// Remove a Flutter package from the local cache
    Remove {
        /// Package name to remove from cache
        pkg: String,
    },
    /// List all cached Flutter packages
    List,
    /// Update cached Flutter packages to latest versions (docs unchanged)
    Update {
        /// Package name to update (omit to update all cached Flutter packages)
        pkg: Option<String>,
    },
}

// ── Stack ─────────────────────────────────────────────────────────────────────

#[derive(Subcommand, Clone, Debug)]
pub enum StackSubcommand {
    /// Interactively create a new custom stack
    New,
    /// Cache all packages in a stack globally (needs internet, run once)
    Install {
        /// Stack name to cache (see 'stack list' for available stacks)
        name: String,
    },
    /// Add a stack to the current project (fully offline)
    Add {
        /// Stack name to apply (packages must already be cached via 'stack install')
        name: String,
    },
    /// List all available stacks
    List,
    /// Show what a stack contains (packages, files, runtime)
    Show {
        /// Stack name to inspect
        name: String,
    },
    /// Delete a custom stack
    Delete {
        /// Stack name to delete
        name: String,
    },
}

// ── Docs ──────────────────────────────────────────────────────────────────────

#[derive(Subcommand, Clone, Debug)]
pub enum DocsSubcommand {
    /// Open a package's cached docs in your editor for editing
    Edit {
        /// Package name whose docs to edit
        pkg: String,
        /// Runtime the package belongs to
        #[arg(long, value_parser = ["bun", "uv", "flutter"])]
        runtime: String,
    },
    /// Print a package's cached docs to the terminal
    Show {
        /// Package name whose docs to show
        pkg: String,
        /// Runtime the package belongs to
        #[arg(long, value_parser = ["bun", "uv", "flutter"])]
        runtime: String,
    },
    /// Reset a package's docs back to the original registry version
    Reset {
        /// Package name whose docs to reset
        pkg: String,
        /// Runtime the package belongs to
        #[arg(long, value_parser = ["bun", "uv", "flutter"])]
        runtime: String,
    },
    /// List all cached documentation files
    List {
        /// Filter by runtime: bun, uv, or flutter
        #[arg(long, value_parser = ["bun", "uv", "flutter"])]
        runtime: Option<String>,
    },
}
