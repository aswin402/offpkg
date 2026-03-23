use anyhow::{anyhow, Result};
use crate::cache::{Cache, compute_sha256};
use crate::config::Config;
use crate::db::{Database, Package};
use crate::tui::{Label, TUI};

/// Check latest version from npm registry
async fn latest_npm(pkg: &str) -> Result<(String, String)> {
    let url = format!("https://registry.npmjs.org/{}/latest", pkg);
    let resp = reqwest::get(&url).await?.json::<serde_json::Value>().await?;
    let version = resp["version"]
        .as_str()
        .ok_or_else(|| anyhow!("No version for '{}'", pkg))?
        .to_string();
    let tarball = resp["dist"]["tarball"]
        .as_str()
        .ok_or_else(|| anyhow!("No tarball for '{}'", pkg))?
        .to_string();
    Ok((version, tarball))
}

/// Check latest version from PyPI
async fn latest_pypi(pkg: &str) -> Result<(String, String)> {
    let url = format!("https://pypi.org/pypi/{}/json", pkg);
    let resp = reqwest::get(&url).await?.json::<serde_json::Value>().await?;
    let version = resp["info"]["version"]
        .as_str()
        .ok_or_else(|| anyhow!("No version for '{}'", pkg))?
        .to_string();
    let empty = vec![];
    let urls = resp["urls"].as_array().unwrap_or(&empty);
    let tarball = urls
        .iter()
        .find(|u| u["filename"].as_str().map(|f| f.ends_with(".whl")).unwrap_or(false))
        .or_else(|| urls.iter().find(|u| u["filename"].as_str().map(|f| f.ends_with(".tar.gz")).unwrap_or(false)))
        .and_then(|u| u["url"].as_str())
        .ok_or_else(|| anyhow!("No download URL for '{}'", pkg))?
        .to_string();
    Ok((version, tarball))
}

/// Check latest version from pub.dev
async fn latest_pubdev(pkg: &str) -> Result<(String, String)> {
    let url = format!("https://pub.dev/api/packages/{}", pkg);
    let resp = reqwest::get(&url).await?.json::<serde_json::Value>().await?;
    let version = resp["latest"]["version"]
        .as_str()
        .ok_or_else(|| anyhow!("No version for '{}'", pkg))?
        .to_string();
    let tarball = format!("https://pub.dev/packages/{}/versions/{}.tar.gz", pkg, version);
    Ok((version, tarball))
}

/// Update a single package in the cache.
/// Never touches docs — user edits are always preserved.
pub async fn update_package(
    tui: &mut TUI,
    db: &Database,
    cache: &Cache,
    pkg: &Package,
) -> Result<bool> {
    // Check latest version from registry
    let (latest_version, tarball_url) = match pkg.runtime.as_str() {
        "bun"     => latest_npm(&pkg.name).await?,
        "uv"      => latest_pypi(&pkg.name).await?,
        "flutter" => latest_pubdev(&pkg.name).await?,
        _         => return Err(anyhow!("Unknown runtime: {}", pkg.runtime)),
    };

    // Already up to date?
    if latest_version == pkg.version {
        tui.print_line(
            Label::Cache,
            &format!("{}@{}", pkg.name, pkg.version),
            Some("already up to date"),
        );
        return Ok(false);
    }

    tui.print_line(
        Label::Resolve,
        &format!("{}: {} → {}", pkg.name, pkg.version, latest_version),
        Some(&pkg.runtime),
    );

    // Download new version
    let new_cache_path = cache.path_for(&pkg.runtime, &pkg.name, &latest_version);
    cache.ensure_dir(&pkg.runtime)?;

    let bar = tui.progress_bar(&format!("downloading {}@{}", pkg.name, latest_version));
    bar.set(0.1, None);
    let size = cache.download_to(&tarball_url, &new_cache_path).await?;
    bar.set(0.9, Some("verifying checksum..."));
    let checksum = compute_sha256(&new_cache_path)?;
    bar.set(1.0, None);
    std::thread::sleep(std::time::Duration::from_millis(150));
    bar.finish("");

    // Delete old cache file
    let old_path = std::path::Path::new(&pkg.cache_path);
    if old_path.exists() {
        std::fs::remove_file(old_path).ok();
    }

    // Delete old DB record and insert new one
    db.delete_package_version(&pkg.name, &pkg.version, &pkg.runtime)?;
    db.insert_package(&Package {
        id: 0,
        name: pkg.name.clone(),
        version: latest_version.clone(),
        runtime: pkg.runtime.clone(),
        cache_path: new_cache_path.to_string_lossy().to_string(),
        checksum,
        size_bytes: Some(size as i64),
        cached_at: chrono::Utc::now().to_rfc3339(),
    })?;

    tui.print_line(
        Label::Install,
        &format!("{}@{} updated", pkg.name, latest_version),
        Some("docs unchanged — run 'offpkg docs reset' to refresh"),
    );

    Ok(true)
}

/// Update all cached packages, or filter by runtime/name.
pub async fn run_update(
    tui: &mut TUI,
    db: &Database,
    cache: &Cache,
    _config: &Config,
    pkg_filter: Option<&str>,
    runtime_filter: Option<&str>,
) -> Result<()> {
    let all = db.list_packages(runtime_filter)?;

    let targets: Vec<Package> = all
        .into_iter()
        .filter(|p| {
            if let Some(name) = pkg_filter {
                p.name == name
            } else {
                true
            }
        })
        .collect();

    if targets.is_empty() {
        tui.print_line(Label::Warn, "no packages found matching filter", None);
        return Ok(());
    }

    let total = targets.len();
    tui.print_line(
        Label::Info,
        &format!("checking {} package{} for updates", total, if total == 1 { "" } else { "s" }),
        Some("docs will NOT be changed"),
    );
    println!();

    let mut updated = 0;
    let mut up_to_date = 0;
    let mut failed: Vec<String> = vec![];

    for pkg in &targets {
        match update_package(tui, db, cache, pkg).await {
            Ok(true)  => updated += 1,
            Ok(false) => up_to_date += 1,
            Err(e) => {
                tui.print_line(
                    Label::Warn,
                    &format!("failed: {}@{}", pkg.name, pkg.version),
                    Some(&e.to_string()),
                );
                failed.push(pkg.name.clone());
            }
        }
    }

    println!();
    tui.print_line(
        Label::Done,
        &format!("{} updated · {} up to date · {} failed", updated, up_to_date, failed.len()),
        None,
    );

    if updated > 0 {
        println!();
        tui.print_line(
            Label::Info,
            "your edited docs are unchanged",
            Some("run 'offpkg docs reset <pkg> --runtime <rt>' to refresh from registry"),
        );
    }

    Ok(())
}