use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "offpkg", version = "0.1.0", about = "Universal offline package manager")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Bun {
        #[command(subcommand)]
        subcmd: BunSubcommand,
    },
    Uv {
        #[command(subcommand)]
        subcmd: UvSubcommand,
    },
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
        /// Specific package to update (omit to update all)
        pkg: Option<String>,
        /// Only update packages for this runtime
        #[arg(long, value_parser = ["bun", "uv", "flutter"])]
        runtime: Option<String>,
    },
    List {
        #[arg(long)]
        runtime: Option<String>,
    },
    Docs {
        #[command(subcommand)]
        subcmd: DocsSubcommand,
    },
    Doctor,
    /// Update offpkg itself to the latest version
    SelfUpdate,
}

#[derive(Subcommand, Clone, Debug)]
pub enum BunSubcommand {
    Add { pkg: String },
    Install { pkg: String },
    Remove { pkg: String },
}

#[derive(Subcommand, Clone, Debug)]
pub enum UvSubcommand {
    Add { pkg: String },
    Install { pkg: String },
    InstallAll,
    Remove { pkg: String },
}

#[derive(Subcommand, Clone, Debug)]
pub enum FlutterSubcommand {
    Add { pkg: String },
    Install { pkg: String },
    InstallAll,
    Remove { pkg: String },
}

#[derive(Subcommand, Clone, Debug)]
pub enum StackSubcommand {
    /// Interactively create a new custom stack
    New,
    /// Cache all packages in a stack globally (needs internet, run once)
    Install {
        name: String,
    },
    /// Add a stack to the current project (fully offline)
    Add {
        name: String,
    },
    /// List all available stacks
    List,
    /// Show what a stack contains
    Show {
        name: String,
    },
    /// Delete a custom stack
    Delete {
        name: String,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum DocsSubcommand {
    Edit {
        pkg: String,
        #[arg(long, value_parser = ["bun", "uv", "flutter"])]
        runtime: String,
    },
    Show {
        pkg: String,
        #[arg(long, value_parser = ["bun", "uv", "flutter"])]
        runtime: String,
    },
    Reset {
        pkg: String,
        #[arg(long, value_parser = ["bun", "uv", "flutter"])]
        runtime: String,
    },
    List {
        #[arg(long)]
        runtime: Option<String>,
    },
}