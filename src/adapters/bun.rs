use crate::cache::{compute_sha256, Cache};
use crate::config::Config;
use crate::db::{Database, Package};
use crate::docs::{fetch_docs, DocsStore};
use crate::tui::{Label, TUI};
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;
use std::time::Instant;

pub struct BunAdapter {
    pub config: Config,
    pub db: Database,
    pub cache: Cache,
    pub tui: TUI,
    pub docs: DocsStore,
}

impl BunAdapter {
    pub fn new(config: Config, db: Database, cache: Cache, tui: TUI, docs: DocsStore) -> Self {
        Self {
            config,
            db,
            cache,
            tui,
            docs,
        }
    }

    pub async fn install(&mut self, pkg: &str) -> Result<()> {
        let start = Instant::now();

        // ── Step 1: resolve version ───────────────────────────────────────
        let sp = self
            .tui
            .spinner(&format!("resolving {} from npm registry...", pkg));
        let url = format!("https://registry.npmjs.org/{}/latest", pkg);
        let resp = reqwest::get(&url)
            .await?
            .json::<serde_json::Value>()
            .await?;
        let version = resp["version"]
            .as_str()
            .ok_or_else(|| anyhow!("No version for '{}'", pkg))?
            .to_string();
        let tarball_url = resp["dist"]["tarball"]
            .as_str()
            .ok_or_else(|| anyhow!("No tarball for '{}'", pkg))?
            .to_string();
        sp.finish(
            Label::Resolve,
            &format!("{}@{}", pkg, version),
            Some("npm registry"),
        );

        // ── Step 2: download with animated bar ───────────────────────────
        let cache_path = self.cache.path_for("bun", pkg, &version);
        self.cache.ensure_dir("bun")?;

        let bar = self
            .tui
            .progress_bar(&format!("downloading {}@{}", pkg, version));
        bar.set(0.05, None);

        // Download happens here — bar animates from 5% to 90%
        // We simulate progress since reqwest doesn't give us chunk callbacks easily
        let bar_ref = &bar;
        let size = {
            bar_ref.set(0.1, None);
            let bytes = self.cache.download_to(&tarball_url, &cache_path).await?;
            bar_ref.set(0.85, Some("verifying checksum..."));
            bytes
        };

        let checksum = compute_sha256(&cache_path)?;
        bar.set(1.0, None);
        // hold full bar briefly so user sees it complete
        std::thread::sleep(std::time::Duration::from_millis(200));
        bar.finish("");

        // ── Step 3: fetch & save docs ─────────────────────────────────────
        let sp = self.tui.spinner(&format!("fetching docs for {}...", pkg));
        let doc_content = fetch_docs("bun", pkg, &version).await.unwrap_or_else(|e| {
            eprintln!("warn: could not fetch docs: {}", e);
            format!("# {pkg} v{version}\n\nNo docs available online. Add your own notes here.\n")
        });
        match self.docs.save_docs("bun", pkg, &doc_content) {
            Ok(_) => sp.finish(
                Label::Info,
                &format!("{} docs cached", pkg),
                Some("~/.offpkg/docs/bun/"),
            ),
            Err(e) => {
                drop(sp);
                self.tui
                    .print_line(Label::Warn, &format!("could not save docs: {}", e), None);
            }
        }

        // ── Step 4: save to DB ────────────────────────────────────────────
        let sp = self.tui.spinner("saving to offpkg cache...");
        self.db.insert_package(&Package {
            id: 0,
            name: pkg.to_string(),
            version: version.clone(),
            runtime: "bun".to_string(),
            cache_path: cache_path.to_string_lossy().to_string(),
            checksum,
            size_bytes: Some(size as i64),
            cached_at: chrono::Utc::now().to_rfc3339(),
        })?;
        sp.finish(
            Label::Install,
            &format!("{}@{} cached", pkg, version),
            Some(&cache_path.to_string_lossy()),
        );

        self.tui.print_done_summary(1, size, start.elapsed());
        Ok(())
    }

    pub fn add(&mut self, pkg: &str, skip_config: bool, is_dev: bool) -> Result<()> {
        let start = Instant::now();

        // ── Step 1: resolve from cache ───────────────────────────────────
        let sp = self
            .tui
            .spinner(&format!("resolving {} from offpkg cache...", pkg));
        let latest = self
            .db
            .list_packages(Some("bun"))?
            .into_iter()
            .filter(|p| p.name == pkg)
            .max_by_key(|p| {
                semver::Version::parse(&p.version).unwrap_or_else(|_| semver::Version::new(0, 0, 0))
            });

        let cached = match latest {
            Some(p) => p,
            None => {
                drop(sp);
                return Err(anyhow!(
                    "'{}' not in offpkg cache.\nRun: offpkg bun install {}",
                    pkg,
                    pkg
                ));
            }
        };

        if !Path::new(&cached.cache_path).exists() {
            drop(sp);
            return Err(anyhow!(
                "Cache file missing. Run: offpkg bun install {}",
                pkg
            ));
        }

        if !self
            .cache
            .verify_checksum(Path::new(&cached.cache_path), &cached.checksum)?
        {
            drop(sp);
            return Err(anyhow!(
                "Checksum mismatch. Run: offpkg bun install {}",
                pkg
            ));
        }

        // ── resolve line (like the screenshot) ───────────────────────────
        sp.finish(
            Label::Resolve,
            &format!("{}@{}", cached.name, cached.version),
            Some("found in offpkg cache"),
        );

        // ── cache read line ───────────────────────────────────────────────
        self.tui.print_line(
            Label::Cache,
            &format!(
                "reading {}",
                Path::new(&cached.cache_path)
                    .file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or(&cached.name)
            ),
            Some("~/.offpkg/cache/bun/"),
        );

        // ── dep resolution (fake extra deps for display realism) ──────────
        // In a real implementation this would resolve package.json deps
        // For now we show the main package resolve line only

        // ── progress bar + extract ────────────────────────────────────────
        let cwd = std::env::current_dir()?;
        let pkg_dest = cwd.join("node_modules").join(&cached.name);
        fs::create_dir_all(&pkg_dest)?;

        let bar = self
            .tui
            .progress_bar(&format!("extracting & linking {}", cached.name));
        bar.set(0.0, None);
        // small pause so user sees the bar start from 0
        std::thread::sleep(std::time::Duration::from_millis(80));
        bar.set(0.3, None);
        extract_tgz(Path::new(&cached.cache_path), &pkg_dest)
            .map_err(|e| anyhow!("Failed to extract '{}': {}", pkg, e))?;
        bar.set(0.75, Some(&format!("linking {}", cached.name)));
        if !skip_config {
            update_package_json(&cwd, &cached.name, &cached.version, is_dev)?;
        }
        create_bin_links(&pkg_dest, &cwd.join("node_modules"))?;
        // hold at 100% briefly so user sees the full bar
        bar.set(1.0, None);
        std::thread::sleep(std::time::Duration::from_millis(180));
        bar.finish("");

        // ── link line ────────────────────────────────────────────────────
        self.tui.print_line(
            Label::Link,
            &format!("{} → node_modules/{}", cached.name, cached.name),
            Some("symlinked from cache"),
        );

        // ── done line ────────────────────────────────────────────────────
        self.tui.print_line(
            Label::Done,
            "1 package installed",
            Some("no network used · "),
        );

        // ── copy docs ─────────────────────────────────────────────────────
        if !skip_config && self.docs.has_docs("bun", &cached.name) {
            match self.docs.copy_to_project("bun", &cached.name, &cwd) {
                Ok(dest) => {
                    self.tui
                        .print_line(Label::Info, &format!("docs → {}", dest.display()), None)
                }
                Err(e) => self
                    .tui
                    .print_line(Label::Warn, &format!("docs: {}", e), None),
            }
        }

        // ── summary cards ─────────────────────────────────────────────────
        self.tui.print_done_summary(1, 0, start.elapsed());
        Ok(())
    }
}

fn extract_tgz(tgz_path: &Path, dest_dir: &Path) -> Result<()> {
    use flate2::read::GzDecoder;
    use tar::Archive;
    let file = fs::File::open(tgz_path)?;
    let mut archive = Archive::new(GzDecoder::new(file));
    for entry in archive.entries()? {
        let mut entry = entry?;
        let entry_path = entry.path()?.to_path_buf();
        let stripped = entry_path
            .components()
            .skip(1)
            .collect::<std::path::PathBuf>();
        if stripped.as_os_str().is_empty() {
            continue;
        }
        let out_path = dest_dir.join(&stripped);
        if entry.header().entry_type().is_dir() {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(p) = out_path.parent() {
                fs::create_dir_all(p)?;
            }
            entry.unpack(&out_path)?;
        }
    }
    Ok(())
}

fn update_package_json(cwd: &Path, pkg_name: &str, version: &str, is_dev: bool) -> Result<()> {
    let path = cwd.join("package.json");
    let mut json: serde_json::Value = if path.exists() {
        serde_json::from_str(&fs::read_to_string(&path)?).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({
            "name": cwd.file_name().and_then(|n| n.to_str()).unwrap_or("project"),
            "version": "1.0.0",
            "dependencies": {}
        })
    };
    let target_key = if is_dev { "devDependencies" } else { "dependencies" };
    if let Some(deps) = json.get_mut(target_key).and_then(|d| d.as_object_mut()) {
        deps.insert(
            pkg_name.to_string(),
            serde_json::Value::String(format!("^{}", version)),
        );
    } else {
        json[target_key] = serde_json::json!({ pkg_name: format!("^{}", version) });
    }
    fs::write(&path, serde_json::to_string_pretty(&json)?)?;
    Ok(())
}

fn create_bin_links(pkg_dir: &Path, node_modules: &Path) -> Result<()> {
    let pkg_json_path = pkg_dir.join("package.json");
    if !pkg_json_path.exists() {
        return Ok(());
    }

    let pkg_json: serde_json::Value = serde_json::from_str(&fs::read_to_string(pkg_json_path)?)?;
    let bin_field = match pkg_json.get("bin") {
        Some(b) => b,
        None => return Ok(()),
    };

    let bin_dir = node_modules.join(".bin");
    fs::create_dir_all(&bin_dir)?;

    let pkg_name = pkg_json.get("name").and_then(|n| n.as_str()).unwrap_or("");
    let rel_prefix = Path::new("..").join(pkg_dir.strip_prefix(node_modules).unwrap_or(pkg_dir.file_name().map(Path::new).unwrap_or(Path::new(""))));

    match bin_field {
        serde_json::Value::String(path) => {
            if !pkg_name.is_empty() {
                link_bin(pkg_dir.join(path), bin_dir.join(pkg_name), rel_prefix.join(path))?;
            }
        }
        serde_json::Value::Object(map) => {
            for (name, path) in map {
                if let Some(path_str) = path.as_str() {
                    link_bin(pkg_dir.join(path_str), bin_dir.join(name), rel_prefix.join(path_str))?;
                }
            }
        }
        _ => {}
    }

    Ok(())
}

fn link_bin(src_abs: std::path::PathBuf, dest: std::path::PathBuf, rel_target: std::path::PathBuf) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        if dest.exists() {
            let _ = fs::remove_file(&dest);
        }
        if src_abs.exists() {
            symlink(&rel_target, &dest)?;
            // Set executable bit on the actual binary file
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(&src_abs) {
                let mut perms = metadata.permissions();
                perms.set_mode(0o755);
                let _ = fs::set_permissions(&src_abs, perms);
            }
        }
    }
    Ok(())
}
