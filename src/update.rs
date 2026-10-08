use crate::cache::{compute_sha256, Cache};
use crate::config::Config;
use crate::db::{Database, Package};
use crate::tui::{Label, TUI};
use anyhow::{anyhow, Result};

/// Check latest version from npm registry
async fn latest_npm(registry: &str, pkg: &str) -> Result<(String, String)> {
    let pkg_encoded = if pkg.starts_with('@') {
        pkg.replacen('/', "%2F", 1)
    } else {
        pkg.to_string()
    };
    let url = format!("{}/{}/latest", registry, pkg_encoded);
    let resp = reqwest::get(&url)
        .await?
        .json::<serde_json::Value>()
        .await?;
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
async fn latest_pypi(registry: &str, pkg: &str) -> Result<(String, String)> {
    let url = format!("{}/pypi/{}/json", registry, pkg);
    let resp = reqwest::get(&url)
        .await?
        .json::<serde_json::Value>()
        .await?;
    let version = resp["info"]["version"]
        .as_str()
        .ok_or_else(|| anyhow!("No version for '{}'", pkg))?
        .to_string();
    let empty = vec![];
    let urls = resp["urls"].as_array().unwrap_or(&empty);
    let tarball = urls
        .iter()
        .find(|u| {
            u["filename"]
                .as_str()
                .map(|f| f.ends_with(".whl"))
                .unwrap_or(false)
        })
        .or_else(|| {
            urls.iter().find(|u| {
                u["filename"]
                    .as_str()
                    .map(|f| f.ends_with(".tar.gz"))
                    .unwrap_or(false)
            })
        })
        .and_then(|u| u["url"].as_str())
        .ok_or_else(|| anyhow!("No download URL for '{}'", pkg))?
        .to_string();
    Ok((version, tarball))
}

/// Check latest version from pub.dev
async fn latest_pubdev(registry: &str, pkg: &str) -> Result<(String, String)> {
    let url = format!("{}/api/packages/{}", registry, pkg);
    let resp = reqwest::get(&url)
        .await?
        .json::<serde_json::Value>()
        .await?;
    let version = resp["latest"]["version"]
        .as_str()
        .ok_or_else(|| anyhow!("No version for '{}'", pkg))?
        .to_string();
    let tarball = format!(
        "{}/packages/{}/versions/{}.tar.gz",
        registry, pkg, version
    );
    Ok((version, tarball))
}

/// Update a single package in the cache.
/// Never touches docs — user edits are always preserved.
pub async fn update_package(
    tui: &mut TUI,
    db: &Database,
    cache: &Cache,
    config: &Config,
    pkg: &Package,
) -> Result<bool> {
    // Check latest version from registry
    let (latest_version, tarball_url) = match pkg.runtime.as_str() {
        "bun" => latest_npm(&config.registries.npm, &pkg.name).await?,
        "uv" => latest_pypi(&config.registries.pypi, &pkg.name).await?,
        "flutter" => latest_pubdev(&config.registries.pubdev, &pkg.name).await?,
        _ => return Err(anyhow!("Unknown runtime: {}", pkg.runtime)),
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
    config: &Config,
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
        &format!(
            "checking {} package{} for updates",
            total,
            if total == 1 { "" } else { "s" }
        ),
        Some("docs will NOT be changed"),
    );
    println!();

    let mut updated = 0;
    let mut up_to_date = 0;
    let mut failed: Vec<String> = vec![];

    for pkg in &targets {
        match update_package(tui, db, cache, config, pkg).await {
            Ok(true) => updated += 1,
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
        &format!(
            "{} updated · {} up to date · {} failed",
            updated,
            up_to_date,
            failed.len()
        ),
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

/// Check if an updated version of offpkg exists on GitHub and run updater if needed
pub async fn run_self_update(tui: &mut TUI, config: &Config, force: bool) -> Result<()> {
    tui.render_logo();
    tui.print_line(Label::Info, "checking for offpkg updates...", None);

    let current_str = env!("CARGO_PKG_VERSION");
    let current_ver = semver::Version::parse(current_str)
        .map_err(|e| anyhow!("Failed to parse current version: {}", e))?;

    let client = reqwest::Client::builder().user_agent("offpkg").build()?;
    let repo = &config.registries.github_repo;

    let remote_url = format!("https://raw.githubusercontent.com/{}/main/Cargo.toml", repo);
    let remote_ver_str = match client
        .get(&remote_url)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                let text = resp.text().await.unwrap_or_default();
                let mut found = None;
                for line in text.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("version") && trimmed.contains('=') {
                        if let Some(val) = trimmed.split('=').nth(1) {
                            found = Some(val.trim().trim_matches('"').trim().to_string());
                            break;
                        }
                    }
                }
                found
            } else {
                None
            }
        }
        Err(_) => None,
    };

    if let Some(ref remote_str) = remote_ver_str {
        if let Ok(remote_ver) = semver::Version::parse(remote_str) {
            if remote_ver <= current_ver && !force {
                tui.print_line(
                    Label::Done,
                    &format!("offpkg is already up to date (v{})", current_str),
                    None,
                );
                return Ok(());
            }

            tui.print_line(
                Label::Info,
                &format!("new version available: v{} -> v{}", current_str, remote_str),
                Some("downloading update..."),
            );
        }
    } else {
        tui.print_line(
            Label::Warn,
            "could not check latest version online",
            Some("running updater script..."),
        );
    }

    let script_cmd = format!(
        "curl -fsSL https://raw.githubusercontent.com/{}/main/update_offpkg.sh | bash",
        repo
    );
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(&script_cmd)
        .status()
        .map_err(|e| anyhow!("Failed to run updater script: {}", e))?;

    if status.success() {
        tui.print_line(
            Label::Done,
            "offpkg update complete",
            Some("restart terminal or reload PATH to use new version"),
        );
    } else {
        tui.print_line(
            Label::Error,
            "update failed",
            Some("check internet connection or try building from source"),
        );
    }

    Ok(())
}
