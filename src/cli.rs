use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "offpkg",
    version = env!("CARGO_PKG_VERSION"),
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
    /// Update cached packages to latest versions, or update offpkg binary itself with --self
    Update {
        /// Specific package name to update, or 'self' to update offpkg (omit to update all)
        pkg: Option<String>,
        /// Only update packages for this runtime
        #[arg(long, value_parser = ["bun", "uv", "flutter"])]
        runtime: Option<String>,
        /// Update offpkg binary itself to the latest version
        #[arg(long = "self", visible_alias = "self-update", short = 's')]
        self_update: bool,
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
    #[command(alias = "selfupdate", alias = "upgrade")]
    SelfUpdate {
        /// Force reinstallation even if already on latest version
        #[arg(long, short = 'f')]
        force: bool,
    },
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
    /// Save current project as a reusable template/stack
    #[command(alias = "export")]
    Save {
        /// Name for the new stack template
        name: String,
        /// Optional description of the template
        #[arg(long, short = 'd')]
        description: Option<String>,
    },
    /// Show filesystem path where templates or a specific stack are stored
    Path {
        /// Stack name (omit to show templates directory)
        name: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_flag() {
        let args = Args::try_parse_from(["offpkg", "-V"]).unwrap();
        assert!(args.version);

        let args = Args::try_parse_from(["offpkg", "--version"]).unwrap();
        assert!(args.version);
    }

    #[test]
    fn test_bun_subcommands() {
        let args = Args::try_parse_from(["offpkg", "bun", "install", "react"]).unwrap();
        match args.command {
            Some(Command::Bun {
                subcmd: BunSubcommand::Install { pkg },
            }) => {
                assert_eq!(pkg, "react");
            }
            _ => panic!("Expected Bun Install"),
        }

        let args = Args::try_parse_from(["offpkg", "bun", "add", "react"]).unwrap();
        match args.command {
            Some(Command::Bun {
                subcmd: BunSubcommand::Add { pkg },
            }) => {
                assert_eq!(pkg, "react");
            }
            _ => panic!("Expected Bun Add"),
        }

        let args = Args::try_parse_from(["offpkg", "bun", "remove", "react"]).unwrap();
        match args.command {
            Some(Command::Bun {
                subcmd: BunSubcommand::Remove { pkg },
            }) => {
                assert_eq!(pkg, "react");
            }
            _ => panic!("Expected Bun Remove"),
        }

        let args = Args::try_parse_from(["offpkg", "bun", "list"]).unwrap();
        assert!(matches!(
            args.command,
            Some(Command::Bun {
                subcmd: BunSubcommand::List
            })
        ));

        let args = Args::try_parse_from(["offpkg", "bun", "update"]).unwrap();
        assert!(matches!(
            args.command,
            Some(Command::Bun {
                subcmd: BunSubcommand::Update { pkg: None }
            })
        ));
    }

    #[test]
    fn test_uv_subcommands() {
        let args = Args::try_parse_from(["offpkg", "uv", "install", "requests"]).unwrap();
        match args.command {
            Some(Command::Uv {
                subcmd: UvSubcommand::Install { pkg },
            }) => {
                assert_eq!(pkg, "requests");
            }
            _ => panic!("Expected Uv Install"),
        }

        let args = Args::try_parse_from(["offpkg", "uv", "add", "requests"]).unwrap();
        match args.command {
            Some(Command::Uv {
                subcmd: UvSubcommand::Add { pkg },
            }) => {
                assert_eq!(pkg, "requests");
            }
            _ => panic!("Expected Uv Add"),
        }

        let args = Args::try_parse_from(["offpkg", "uv", "install-all"]).unwrap();
        assert!(matches!(
            args.command,
            Some(Command::Uv {
                subcmd: UvSubcommand::InstallAll
            })
        ));
    }

    #[test]
    fn test_flutter_subcommands() {
        let args = Args::try_parse_from(["offpkg", "flutter", "install", "dio"]).unwrap();
        match args.command {
            Some(Command::Flutter {
                subcmd: FlutterSubcommand::Install { pkg },
            }) => {
                assert_eq!(pkg, "dio");
            }
            _ => panic!("Expected Flutter Install"),
        }

        let args = Args::try_parse_from(["offpkg", "flutter", "add", "dio"]).unwrap();
        match args.command {
            Some(Command::Flutter {
                subcmd: FlutterSubcommand::Add { pkg },
            }) => {
                assert_eq!(pkg, "dio");
            }
            _ => panic!("Expected Flutter Add"),
        }

        let args = Args::try_parse_from(["offpkg", "flutter", "install-all"]).unwrap();
        assert!(matches!(
            args.command,
            Some(Command::Flutter {
                subcmd: FlutterSubcommand::InstallAll
            })
        ));
    }

    #[test]
    fn test_stack_subcommands() {
        let args = Args::try_parse_from(["offpkg", "stack", "list"]).unwrap();
        assert!(matches!(
            args.command,
            Some(Command::Stack {
                subcmd: StackSubcommand::List
            })
        ));

        let args = Args::try_parse_from(["offpkg", "stack", "show", "react-vite"]).unwrap();
        match args.command {
            Some(Command::Stack {
                subcmd: StackSubcommand::Show { name },
            }) => {
                assert_eq!(name, "react-vite");
            }
            _ => panic!("Expected Stack Show"),
        }

        let args = Args::try_parse_from(["offpkg", "stack", "new"]).unwrap();
        assert!(matches!(
            args.command,
            Some(Command::Stack {
                subcmd: StackSubcommand::New
            })
        ));

        let args = Args::try_parse_from(["offpkg", "stack", "save", "my-app"]).unwrap();
        match args.command {
            Some(Command::Stack {
                subcmd: StackSubcommand::Save { name, description },
            }) => {
                assert_eq!(name, "my-app");
                assert!(description.is_none());
            }
            _ => panic!("Expected Stack Save"),
        }

        let args = Args::try_parse_from(["offpkg", "stack", "path"]).unwrap();
        match args.command {
            Some(Command::Stack {
                subcmd: StackSubcommand::Path { name },
            }) => {
                assert!(name.is_none());
            }
            _ => panic!("Expected Stack Path"),
        }
    }

    #[test]
    fn test_docs_subcommands() {
        let args =
            Args::try_parse_from(["offpkg", "docs", "edit", "react", "--runtime", "bun"]).unwrap();
        match args.command {
            Some(Command::Docs {
                subcmd: DocsSubcommand::Edit { pkg, runtime },
            }) => {
                assert_eq!(pkg, "react");
                assert_eq!(runtime, "bun");
            }
            _ => panic!("Expected Docs Edit"),
        }

        let args =
            Args::try_parse_from(["offpkg", "docs", "show", "fastapi", "--runtime", "uv"]).unwrap();
        match args.command {
            Some(Command::Docs {
                subcmd: DocsSubcommand::Show { pkg, runtime },
            }) => {
                assert_eq!(pkg, "fastapi");
                assert_eq!(runtime, "uv");
            }
            _ => panic!("Expected Docs Show"),
        }
    }

    #[test]
    fn test_general_and_validation() {
        let args = Args::try_parse_from(["offpkg", "doctor"]).unwrap();
        assert!(matches!(args.command, Some(Command::Doctor)));

        let args = Args::try_parse_from(["offpkg", "self-update"]).unwrap();
        assert!(matches!(
            args.command,
            Some(Command::SelfUpdate { force: false })
        ));

        let args = Args::try_parse_from(["offpkg", "selfupdate", "--force"]).unwrap();
        assert!(matches!(
            args.command,
            Some(Command::SelfUpdate { force: true })
        ));

        let args = Args::try_parse_from(["offpkg", "upgrade"]).unwrap();
        assert!(matches!(
            args.command,
            Some(Command::SelfUpdate { force: false })
        ));

        let args = Args::try_parse_from(["offpkg", "update", "--self"]).unwrap();
        assert!(matches!(
            args.command,
            Some(Command::Update {
                pkg: None,
                runtime: None,
                self_update: true
            })
        ));

        let args = Args::try_parse_from(["offpkg", "update", "self"]).unwrap();
        assert!(matches!(
            args.command,
            Some(Command::Update {
                pkg: Some(ref p),
                runtime: None,
                self_update: false
            }) if p == "self"
        ));

        let args = Args::try_parse_from(["offpkg", "list", "--runtime", "bun"]).unwrap();
        match args.command {
            Some(Command::List { runtime }) => assert_eq!(runtime, Some("bun".to_string())),
            _ => panic!("Expected List"),
        }

        // Invalid runtime should fail parsing
        let result = Args::try_parse_from(["offpkg", "list", "--runtime", "unknown"]);
        assert!(result.is_err());
    }
}
