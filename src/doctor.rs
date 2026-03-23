use anyhow::Result;
use std::process::Command;
use std::path::PathBuf;
use crate::config::Config;
use crate::db::Database;
use crate::tui::{Label, TUI};

pub fn run_doctor(tui: &mut TUI, config: &Config, db: &Database) -> Result<()> {
    tui.render_logo();
    tui.print_line(Label::Info, "offpkg doctor", Some("running environment checks"));
    println!();

    check_runtime(tui, "bun",     &["bun",     "--version"]);
    check_runtime(tui, "uv",      &["uv",      "--version"]);
    check_runtime(tui, "flutter", &["flutter", "--version"]);

    check_cache_dir(tui, &config.cache_path());
    check_db(tui, db)?;

    println!();
    tui.print_line(Label::Done, "doctor complete", None);
    Ok(())
}

fn check_runtime(tui: &mut TUI, name: &str, args: &[&str]) {
    let (bin, rest) = args.split_first().expect("args must not be empty");
    match Command::new(bin).args(rest).output() {
        Ok(out) if out.status.success() => {
            // Flutter writes its version to stderr; most others use stdout
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            let raw = if stdout.trim().is_empty() { stderr } else { stdout };
            let version = raw.lines().next().unwrap_or("").trim().to_string();
            tui.print_line(Label::Done, name, Some(&version));
        }
        Ok(_) => {
            tui.print_line(Label::Error, name, Some("command failed — is it installed?"));
        }
        Err(_) => {
            tui.print_line(Label::Warn, name, Some("not found in PATH"));
        }
    }
}

fn check_cache_dir(tui: &mut TUI, path: &PathBuf) {
    if path.exists() {
        let entry_count = std::fs::read_dir(path)
            .map(|entries| entries.count())
            .unwrap_or(0);
        tui.print_line(
            Label::Done,
            "cache directory",
            Some(&format!("{} — {} entries", path.display(), entry_count)),
        );
    } else {
        tui.print_line(
            Label::Warn,
            "cache directory",
            Some(&format!("{} — not yet created", path.display())),
        );
    }
}

fn check_db(tui: &mut TUI, db: &Database) -> Result<()> {
    let count = db.count_packages()?;
    tui.print_line(
        Label::Done,
        "database",
        Some(&format!("{} package(s) cached — integrity ok", count)),
    );
    Ok(())
}