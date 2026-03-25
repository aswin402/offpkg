use crate::db::Database;
use crate::tui::{Label, TUI};
use anyhow::{anyhow, Result};
use std::fs;

/// Remove a package from the offpkg cache.
/// Deletes the file from disk and the DB record.
/// Does NOT touch the project (node_modules / .venv / pubspec.yaml).
pub fn remove_from_cache(tui: &mut TUI, db: &Database, pkg: &str, runtime: &str) -> Result<()> {
    // spinner() takes &self so calling it on &mut TUI is fine
    let spinner = tui.spinner(&format!("looking up {} in offpkg cache...", pkg));

    let packages = db.find_packages(pkg, runtime)?;

    if packages.is_empty() {
        drop(spinner);
        return Err(anyhow!(
            "'{}' ({}) is not in the offpkg cache — nothing to remove.",
            pkg,
            runtime
        ));
    }

    drop(spinner);

    let mut total_freed: u64 = 0;

    for p in &packages {
        let path = std::path::Path::new(&p.cache_path);
        if path.exists() {
            let size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
            fs::remove_file(path).map_err(|e| anyhow!("Failed to delete {:?}: {}", path, e))?;
            total_freed += size;
            tui.print_line(
                Label::Cache,
                &format!("deleted {}@{}", p.name, p.version),
                Some(&p.cache_path),
            );
        } else {
            tui.print_line(
                Label::Warn,
                &format!("cache file already missing for {}@{}", p.name, p.version),
                None,
            );
        }

        db.delete_package_version(&p.name, &p.version, runtime)?;
    }

    let freed_str = if total_freed < 1_000_000 {
        format!("{:.1} KB freed", total_freed as f64 / 1_000.0)
    } else {
        format!("{:.1} MB freed", total_freed as f64 / 1_000_000.0)
    };

    tui.print_line(
        Label::Done,
        &format!("{} removed from offpkg cache", pkg),
        Some(&freed_str),
    );
    tui.print_line(
        Label::Info,
        "note: your project files were not changed",
        Some("only the offpkg cache was updated"),
    );

    Ok(())
}
