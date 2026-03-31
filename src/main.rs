pub mod cache;
pub mod cli;
pub mod config;
pub mod db;
pub mod docs;
pub mod doctor;
pub mod remove;
pub mod stacks;
pub mod tui;
pub mod update;
pub mod adapters {
    pub mod bun;
    pub mod flutter;
    pub mod uv;
}

use crate::adapters::bun::BunAdapter;
use crate::adapters::flutter::FlutterAdapter;
use crate::adapters::uv::UvAdapter;
use crate::cache::Cache;
use crate::cli::{
    Args, BunSubcommand, Command, DocsSubcommand, FlutterSubcommand, StackSubcommand, UvSubcommand,
};
use crate::config::Config;
use crate::db::Database;
use crate::docs::{fetch_docs, DocsStore};
use crate::stacks::StackStore;
use crate::tui::{Label, TUI};
use crate::update::run_update;
use anyhow::{anyhow, Context, Result};
use clap::{CommandFactory, Parser};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Args::parse();
    let config = Config::load().context("Failed to load config")?;
    let mut tui = TUI::init(config.clone()).context("Failed to init TUI")?;
    let db = Database::open(&config).context("Failed to open DB")?;
    let cache = Cache::new(config.clone());
    let docs_store = DocsStore::new(config.clone());
    let stack_store = StackStore::new(config.clone());

    if cli.version {
        tui.render_logo();
        return Ok(());
    }

    let command = match cli.command {
        Some(cmd) => cmd,
        None => {
            Args::command().print_help()?;
            println!();
            return Ok(());
        }
    };

    match command {
        Command::List { runtime } => {
            tui.render_logo();
            let pkgs = db.list_packages(runtime.as_deref())?;
            tui.print_line(
                Label::Info,
                &format!("{} packages found:", pkgs.len()),
                None,
            );
            for pkg in pkgs {
                tui.print_line(
                    Label::Done,
                    &format!("{}@{} ({})", pkg.name, pkg.version, pkg.runtime),
                    Some(&pkg.cache_path),
                );
            }
        }

        Command::Doctor => {
            doctor::run_doctor(&mut tui, &config, &db)?;
        }

        // ── Stack ─────────────────────────────────────────────────────────
        Command::Stack { subcmd } => match subcmd {
            StackSubcommand::List => {
                tui.render_logo();
                tui.print_line(Label::Info, "available stacks", None);
                println!();
                for stack in stack_store.all_stacks() {
                    let total = stack.packages.len() + stack.dev_packages.len();
                    let files = stack.files.len();
                    tui.print_line(
                        Label::Done,
                        &format!("{}", stack.name),
                        Some(&format!(
                            "[{}]  {} packages  {} files  — {}",
                            stack.runtime, total, files, stack.description
                        )),
                    );
                }
            }

            StackSubcommand::Show { name } => {
                let stack = stack_store
                    .find(&name)
                    .ok_or_else(|| anyhow!("Stack '{}' not found. Run: offpkg stack list", name))?;

                println!();
                tui.print_line(
                    Label::Info,
                    &format!("stack: {}", stack.name),
                    Some(&stack.description),
                );
                tui.print_line(Label::Info, "runtime", Some(&stack.runtime));
                println!();
                tui.print_line(Label::Resolve, "packages", None);
                for pkg in &stack.packages {
                    println!("    {}", pkg);
                }
                if !stack.dev_packages.is_empty() {
                    tui.print_line(Label::Resolve, "dev packages", None);
                    for pkg in &stack.dev_packages {
                        println!("    {}", pkg);
                    }
                }
                if !stack.files.is_empty() {
                    println!();
                    tui.print_line(Label::Cache, "files generated", None);
                    for f in &stack.files {
                        println!("    {}", f.path);
                    }
                }
            }

            StackSubcommand::Install { name } => {
                let stack = stack_store
                    .find(&name)
                    .ok_or_else(|| anyhow!("Stack '{}' not found. Run: offpkg stack list", name))?;

                let total = stack.packages.len() + stack.dev_packages.len();
                tui.print_line(
                    Label::Info,
                    &format!("installing stack: {}", stack.name),
                    Some(&format!("{} packages to cache", total)),
                );
                println!();

                let all_pkgs: Vec<String> = stack
                    .packages
                    .iter()
                    .chain(stack.dev_packages.iter())
                    .cloned()
                    .collect();

                let mut success = 0;
                let mut failed: Vec<String> = vec![];

                for pkg in &all_pkgs {
                    // Skip if already cached
                    let already = db
                        .list_packages(Some(&stack.runtime))?
                        .into_iter()
                        .any(|p| p.name == *pkg);

                    if already {
                        tui.print_line(Label::Cache, pkg, Some("already cached, skipping"));
                        success += 1;
                        continue;
                    }

                    let result = match stack.runtime.as_str() {
                        "bun" => {
                            let mut a = BunAdapter::new(
                                config.clone(),
                                db.clone(),
                                cache.clone(),
                                tui.clone(),
                                docs_store.clone(),
                            );
                            a.install(pkg).await
                        }
                        "uv" => {
                            let mut a = UvAdapter::new(
                                config.clone(),
                                db.clone(),
                                cache.clone(),
                                tui.clone(),
                                docs_store.clone(),
                            );
                            a.install(pkg).await
                        }
                        "flutter" => {
                            let mut a = FlutterAdapter::new(
                                config.clone(),
                                db.clone(),
                                cache.clone(),
                                tui.clone(),
                                docs_store.clone(),
                            );
                            a.install(pkg).await
                        }
                        _ => Err(anyhow!("Unknown runtime: {}", stack.runtime)),
                    };

                    match result {
                        Ok(_) => success += 1,
                        Err(e) => {
                            tui.print_line(
                                Label::Warn,
                                &format!("failed: {}", pkg),
                                Some(&e.to_string()),
                            );
                            failed.push(pkg.clone());
                        }
                    }
                }

                println!();
                tui.print_line(
                    Label::Done,
                    &format!("stack {} cached", stack.name),
                    Some(&format!("{}/{} packages", success, total)),
                );
                if !failed.is_empty() {
                    tui.print_line(
                        Label::Warn,
                        "some packages failed",
                        Some(&failed.join(", ")),
                    );
                }
                tui.print_line(
                    Label::Info,
                    "run when offline:",
                    Some(&format!("offpkg stack add {}", name)),
                );
            }

            StackSubcommand::Add { name } => {
                let stack = stack_store
                    .find(&name)
                    .ok_or_else(|| anyhow!("Stack '{}' not found. Run: offpkg stack list", name))?;

                let total = stack.packages.len() + stack.dev_packages.len();
                tui.print_line(
                    Label::Info,
                    &format!("adding stack: {}", stack.name),
                    Some(&format!("{} packages + {} files", total, stack.files.len())),
                );
                println!();

                let cwd = std::env::current_dir()?;

                // Write config files
                println!();
                tui.print_line(Label::Cache, "writing config files", None);
                let created = stack_store.write_files(&stack, &cwd)?;
                for f in &created {
                    tui.print_line(Label::Link, f, Some("created"));
                }
                if created.is_empty() {
                    tui.print_line(
                        Label::Info,
                        "all config files already exist",
                        Some("skipped"),
                    );
                }

                let mut success = 0;
                let mut failed: Vec<String> = vec![];

                let mut run_add = |pkg: &String, skip: bool, dev: bool| {
                    let result = match stack.runtime.as_str() {
                        "bun" => {
                            let mut a = BunAdapter::new(
                                config.clone(), db.clone(), cache.clone(), tui.clone(), docs_store.clone(),
                            );
                            a.add(pkg, skip, dev)
                        }
                        "uv" => {
                            let mut a = UvAdapter::new(
                                config.clone(), db.clone(), cache.clone(), tui.clone(), docs_store.clone(),
                            );
                            a.add(pkg, skip, dev)
                        }
                        "flutter" => {
                            let mut a = FlutterAdapter::new(
                                config.clone(), db.clone(), cache.clone(), tui.clone(), docs_store.clone(),
                            );
                            a.add(pkg, skip, dev)
                        }
                        _ => Err(anyhow!("Unknown runtime: {}", stack.runtime)),
                    };

                    match result {
                        Ok(_) => success += 1,
                        Err(e) => {
                            tui.print_line(
                                Label::Warn,
                                &format!("failed: {}", pkg),
                                Some(&e.to_string()),
                            );
                            failed.push(pkg.clone());
                        }
                    }
                };

                for pkg in &stack.packages { run_add(pkg, false, false); }
                for pkg in &stack.dev_packages { run_add(pkg, false, true); }
                for pkg in &stack.transitive_packages { run_add(pkg, true, false); }

                println!();
                tui.print_line(
                    Label::Done,
                    &format!("stack {} ready", stack.name),
                    Some(&format!(
                        "{}/{} packages · {} files",
                        success,
                        total,
                        created.len()
                    )),
                );

                if !failed.is_empty() {
                    tui.print_line(
                        Label::Warn,
                        "some packages failed",
                        Some(&failed.join(", ")),
                    );
                    tui.print_line(
                        Label::Info,
                        "run install first:",
                        Some(&format!("offpkg stack install {}", name)),
                    );
                }
            }

            StackSubcommand::New => {
                stack_store.create_interactive()?;
            }

            StackSubcommand::Delete { name } => match stack_store.delete(&name) {
                Ok(_) => {
                    tui.print_line(Label::Done, &format!("stack '{}' deleted", name), None);
                }
                Err(e) => {
                    tui.print_line(Label::Error, &e.to_string(), None);
                }
            },
        },

        // ── Docs ──────────────────────────────────────────────────────────
        Command::Docs { subcmd } => match subcmd {
            DocsSubcommand::Edit { pkg, runtime } => {
                tui.print_line(
                    Label::Info,
                    &format!("opening ~/.offpkg/docs/{}/{}.md", runtime, pkg),
                    Some("edits apply to all future project copies"),
                );
                docs_store.open_in_editor(&runtime, &pkg)?;
                tui.print_line(
                    Label::Done,
                    "saved",
                    Some(&format!("~/.offpkg/docs/{}/{}.md", runtime, pkg)),
                );
            }
            DocsSubcommand::Show { pkg, runtime } => {
                println!("{}", docs_store.read_docs(&runtime, &pkg)?);
            }
            DocsSubcommand::Reset { pkg, runtime } => {
                let sp = tui.spinner(&format!("fetching original {} docs...", pkg));
                let version = db
                    .list_packages(Some(&runtime))?
                    .into_iter()
                    .filter(|p| p.name == pkg)
                    .max_by_key(|p| {
                        semver::Version::parse(&p.version)
                            .unwrap_or_else(|_| semver::Version::new(0, 0, 0))
                    })
                    .map(|p| p.version)
                    .unwrap_or_else(|| "latest".to_string());
                let content = fetch_docs(&runtime, &pkg, &version)
                    .await
                    .unwrap_or_else(|_| format!("# {pkg}\n\nNo docs available.\n"));
                docs_store.reset_global_doc(&runtime, &pkg, &content)?;
                sp.finish(
                    Label::Done,
                    &format!("~/.offpkg/docs/{}/{}.md reset", runtime, pkg),
                    None,
                );
            }
            DocsSubcommand::List { runtime } => {
                tui.render_logo();
                let base = config.cache_path().join("docs");
                let runtimes = if let Some(rt) = runtime {
                    vec![rt]
                } else {
                    vec!["bun".to_string(), "uv".to_string(), "flutter".to_string()]
                };
                let mut total = 0;
                for rt in &runtimes {
                    let dir = base.join(rt);
                    if dir.exists() {
                        for entry in std::fs::read_dir(&dir)? {
                            let entry = entry?;
                            let name = entry.file_name().to_string_lossy().replace(".md", "");
                            tui.print_line(Label::Done, &format!("{} ({})", name, rt), None);
                            total += 1;
                        }
                    }
                }
                tui.print_line(Label::Info, &format!("{} docs cached", total), None);
            }
        },

        // ── Bun ───────────────────────────────────────────────────────────
        Command::Bun { subcmd } => match subcmd {
            BunSubcommand::Add { pkg } => {
                let mut a = BunAdapter::new(
                    config.clone(),
                    db.clone(),
                    cache.clone(),
                    tui.clone(),
                    docs_store,
                );
                a.add(&pkg, false, false)?;
            }
            BunSubcommand::Install { pkg } => {
                let mut a = BunAdapter::new(
                    config.clone(),
                    db.clone(),
                    cache.clone(),
                    tui.clone(),
                    docs_store,
                );
                a.install(&pkg).await?;
            }
            BunSubcommand::Remove { pkg } => {
                remove::remove_from_cache(&mut tui, &db, &pkg, "bun")?;
            }
            BunSubcommand::List => {
                tui.render_logo();
                let pkgs = db.list_packages(Some("bun"))?;
                tui.print_line(
                    Label::Info,
                    &format!("{} bun packages cached:", pkgs.len()),
                    None,
                );
                for pkg in pkgs {
                    tui.print_line(
                        Label::Done,
                        &format!("{}@{}", pkg.name, pkg.version),
                        Some(&pkg.cache_path),
                    );
                }
            }
            BunSubcommand::Update { pkg } => {
                run_update(&mut tui, &db, &cache, &config, pkg.as_deref(), Some("bun")).await?;
            }
        },

        // ── UV ────────────────────────────────────────────────────────────
        Command::Uv { subcmd } => match subcmd {
            UvSubcommand::Add { pkg } => {
                let mut a = UvAdapter::new(
                    config.clone(),
                    db.clone(),
                    cache.clone(),
                    tui.clone(),
                    docs_store,
                );
                a.add(&pkg, false, false)?;
            }
            UvSubcommand::Install { pkg } => {
                let mut a = UvAdapter::new(
                    config.clone(),
                    db.clone(),
                    cache.clone(),
                    tui.clone(),
                    docs_store,
                );
                a.install(&pkg).await?;
            }
            UvSubcommand::InstallAll => {
                let mut a = UvAdapter::new(
                    config.clone(),
                    db.clone(),
                    cache.clone(),
                    tui.clone(),
                    docs_store,
                );
                a.install_all().await?;
            }
            UvSubcommand::Remove { pkg } => {
                remove::remove_from_cache(&mut tui, &db, &pkg, "uv")?;
            }
            UvSubcommand::List => {
                tui.render_logo();
                let pkgs = db.list_packages(Some("uv"))?;
                tui.print_line(
                    Label::Info,
                    &format!("{} uv packages cached:", pkgs.len()),
                    None,
                );
                for pkg in pkgs {
                    tui.print_line(
                        Label::Done,
                        &format!("{}@{}", pkg.name, pkg.version),
                        Some(&pkg.cache_path),
                    );
                }
            }
            UvSubcommand::Update { pkg } => {
                run_update(&mut tui, &db, &cache, &config, pkg.as_deref(), Some("uv")).await?;
            }
        },

        // ── Flutter ───────────────────────────────────────────────────────
        Command::Flutter { subcmd } => match subcmd {
            FlutterSubcommand::Add { pkg } => {
                let mut a = FlutterAdapter::new(
                    config.clone(),
                    db.clone(),
                    cache.clone(),
                    tui.clone(),
                    docs_store,
                );
                a.add(&pkg, false, false)?;
            }
            FlutterSubcommand::Install { pkg } => {
                let mut a = FlutterAdapter::new(
                    config.clone(),
                    db.clone(),
                    cache.clone(),
                    tui.clone(),
                    docs_store,
                );
                a.install(&pkg).await?;
            }
            FlutterSubcommand::InstallAll => {
                let mut a = FlutterAdapter::new(
                    config.clone(),
                    db.clone(),
                    cache.clone(),
                    tui.clone(),
                    docs_store,
                );
                a.install_all().await?;
            }
            FlutterSubcommand::Remove { pkg } => {
                remove::remove_from_cache(&mut tui, &db, &pkg, "flutter")?;
            }
            FlutterSubcommand::List => {
                tui.render_logo();
                let pkgs = db.list_packages(Some("flutter"))?;
                tui.print_line(
                    Label::Info,
                    &format!("{} flutter packages cached:", pkgs.len()),
                    None,
                );
                for pkg in pkgs {
                    tui.print_line(
                        Label::Done,
                        &format!("{}@{}", pkg.name, pkg.version),
                        Some(&pkg.cache_path),
                    );
                }
            }
            FlutterSubcommand::Update { pkg } => {
                run_update(&mut tui, &db, &cache, &config, pkg.as_deref(), Some("flutter")).await?;
            }
        },

        Command::SelfUpdate => {
            tui.print_line(Label::Info, "updating offpkg...", None);
            let status = std::process::Command::new("sh")
                .arg("-c")
                .arg("curl -fsSL https://raw.githubusercontent.com/YOUR_USERNAME/offpkg/main/install.sh | bash")
                .status()
                .map_err(|e| anyhow::anyhow!("Failed to run updater: {}", e))?;
            if status.success() {
                tui.print_line(
                    Label::Done,
                    "offpkg updated",
                    Some("restart terminal to use new version"),
                );
            } else {
                tui.print_line(
                    Label::Error,
                    "update failed",
                    Some("check your internet connection"),
                );
            }
        }

        Command::Update { pkg, runtime } => {
            run_update(
                &mut tui,
                &db,
                &cache,
                &config,
                pkg.as_deref(),
                runtime.as_deref(),
            )
            .await?;
        }
    }

    tui.cleanup()?;
    Ok(())
}
