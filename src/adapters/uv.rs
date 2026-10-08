use crate::cache::{compute_sha256, Cache};
use crate::config::Config;
use crate::db::{Database, Package};
use crate::docs::{fetch_docs, DocsStore};
use crate::package_spec::PackageSpec;
use crate::tui::{Label, TUI};
use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::Command;
use std::time::Instant;

pub struct UvAdapter {
    pub config: Config,
    pub db: Database,
    pub cache: Cache,
    pub tui: TUI,
    pub docs: DocsStore,
}

impl UvAdapter {
    pub fn new(config: Config, db: Database, cache: Cache, tui: TUI, docs: DocsStore) -> Self {
        Self {
            config,
            db,
            cache,
            tui,
            docs,
        }
    }

    pub fn add(&mut self, pkg_spec_str: &str, skip_config: bool, _is_dev: bool) -> Result<()> {
        let start = Instant::now();
        let spec = PackageSpec::parse(pkg_spec_str, "uv")?;
        let pkg = &spec.name;

        let sp = self
            .tui
            .spinner(&format!("resolving {} from offpkg cache...", pkg_spec_str));
        let latest = self
            .db
            .list_packages(Some("uv"))?
            .into_iter()
            .filter(|p| p.name == *pkg)
            .filter(|p| {
                if let Some(ref req_ver) = spec.version_req {
                    p.version == *req_ver || p.version.starts_with(req_ver.trim_start_matches("=="))
                } else {
                    true
                }
            })
            .max_by_key(|p| {
                semver::Version::parse(&p.version).unwrap_or_else(|_| semver::Version::new(0, 0, 0))
            });

        let cached = match latest {
            Some(p) => p,
            None => {
                drop(sp);
                return Err(anyhow!(
                    "'{}' is not in the offpkg cache.\nRun: offpkg uv install {}",
                    pkg_spec_str,
                    pkg_spec_str
                ));
            }
        };

        if !Path::new(&cached.cache_path).exists() {
            drop(sp);
            return Err(anyhow!(
                "Cache file missing. Re-run: offpkg uv install {}",
                pkg_spec_str
            ));
        }

        if !self
            .cache
            .verify_checksum(Path::new(&cached.cache_path), &cached.checksum)?
        {
            drop(sp);
            return Err(anyhow!(
                "Checksum mismatch. Re-run: offpkg uv install {}",
                pkg
            ));
        }
        sp.finish(
            Label::Resolve,
            &format!("{}@{}", cached.name, cached.version),
            Some("found in offpkg cache"),
        );

        self.tui.print_line(
            Label::Cache,
            &format!(
                "reading {}",
                Path::new(&cached.cache_path)
                    .file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or(&cached.name)
            ),
            Some("~/.offpkg/cache/uv/"),
        );

        let cache_dir = self.config.cache_path().join("uv");
        let bar = self.tui.progress_bar("running uv add --frozen --no-index");
        bar.set(0.3, None);

        let output = Command::new("uv")
            .args([
                "add",
                "--frozen",
                "--no-index",
                "--find-links",
                &cache_dir.to_string_lossy(),
                pkg,
            ])
            .output()
            .map_err(|e| anyhow!("Failed to run uv: {}", e))?;

        bar.set(1.0, None);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            drop(bar);
            return Err(anyhow!(
                "uv add failed for '{}'.\n{}\n\nTip: offpkg uv install-all",
                pkg,
                stderr.trim()
            ));
        }

        bar.finish(&format!("{} added to .venv", pkg));

        // ── copy docs into project ────────────────────────────────────────
        if !skip_config && self.docs.has_docs("uv", &cached.name) {
            let cwd = std::env::current_dir()?;
            let sp = self.tui.spinner("copying docs to project...");
            self.docs.copy_to_project("uv", &cached.name, &cwd)?;
            sp.finish(
                Label::Info,
                &format!("docs → offpkg_docs/{}.md", cached.name),
                None,
            );
        }

        self.tui.print_line(
            Label::Link,
            &format!("{} → .venv", cached.name),
            Some(&format!("v{}", cached.version)),
        );
        self.tui
            .print_line(Label::Done, "1 package installed", Some("no network used"));
        self.tui.print_done_summary(1, 0, start.elapsed());
        Ok(())
    }

    pub async fn install(&mut self, pkg_spec_str: &str) -> Result<()> {
        let start = Instant::now();
        let spec = PackageSpec::parse(pkg_spec_str, "uv")?;
        let pkg = &spec.name;

        let sp = self.tui.spinner(&format!("resolving {} from PyPI...", pkg_spec_str));
        let url = format!("{}/pypi/{}/json", self.config.registries.pypi, pkg);
        let resp = reqwest::get(&url)
            .await?
            .json::<serde_json::Value>()
            .await?;

        let (version, tarball_url) = if let Some(ref ver) = spec.version_req {
            let releases = resp["releases"]
                .as_object()
                .ok_or_else(|| anyhow!("No releases found for '{}'", pkg))?;
            let ver_clean = ver.trim_start_matches("==").trim_start_matches(">=").trim();
            if let Some(files) = releases.get(ver_clean).and_then(|f| f.as_array()) {
                let download_url = select_pypi_download_url(files, pkg)?;
                (ver_clean.to_string(), download_url)
            } else {
                return Err(anyhow!("Version '{}' not found on PyPI for '{}'", ver, pkg));
            }
        } else {
            let version = resp["info"]["version"]
                .as_str()
                .ok_or_else(|| anyhow!("No version in PyPI response for '{}'", pkg))?
                .to_string();
            let empty = vec![];
            let urls = resp["urls"].as_array().unwrap_or(&empty);
            let download_url = select_pypi_download_url(urls, pkg)?;
            (version, download_url)
        };

        sp.finish(
            Label::Resolve,
            &format!("{}@{}", pkg, version),
            Some("PyPI"),
        );

        let cache_path = self.cache.path_for("uv", pkg, &version);
        self.cache.ensure_dir("uv")?;

        let bar = self
            .tui
            .progress_bar(&format!("downloading {}@{}", pkg, version));
        bar.set(0.1, None);
        let size = self.cache.download_to(&tarball_url, &cache_path).await?;
        bar.set(0.9, Some("verifying checksum..."));
        let checksum = compute_sha256(&cache_path)?;
        bar.set(1.0, None);
        bar.finish(&format!(
            "{}@{} downloaded  ({:.1} KB)",
            pkg,
            version,
            size as f64 / 1_000.0
        ));

        // ── fetch & save docs ─────────────────────────────────────────────
        let sp = self.tui.spinner(&format!("fetching docs for {}...", pkg));
        let doc_content = fetch_docs("uv", pkg, &version)
            .await
            .unwrap_or_else(|_| format!("# {pkg} v{version}\n\nNo docs available.\n"));
        self.docs.save_docs("uv", pkg, &doc_content)?;
        sp.finish(
            Label::Info,
            &format!("{} docs cached", pkg),
            Some("~/.offpkg/docs/uv/"),
        );

        let sp = self.tui.spinner("saving to offpkg cache...");
        self.db.insert_package(&Package {
            id: 0,
            name: pkg.to_string(),
            version: version.clone(),
            runtime: "uv".to_string(),
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

    pub async fn install_all(&mut self) -> Result<()> {
        let cwd = std::env::current_dir()?;
        let pyproject_path = cwd.join("pyproject.toml");

        if !pyproject_path.exists() {
            return Err(anyhow!(
                "No pyproject.toml found. Run from inside your Python project."
            ));
        }

        let content = std::fs::read_to_string(&pyproject_path)?;
        let pkg_names = parse_pyproject_deps(&content)?;

        self.tui.print_line(
            Label::Info,
            &format!("found {} dependencies in pyproject.toml", pkg_names.len()),
            None,
        );

        let mut cached_pkgs: std::collections::HashSet<String> = self
            .db
            .list_packages(Some("uv"))?
            .into_iter()
            .map(|p| p.name)
            .collect();

        let mut success = 0;
        let mut failed: Vec<String> = vec![];

        for pkg in &pkg_names {
            let already = cached_pkgs.contains(pkg);
            if already {
                self.tui
                    .print_line(Label::Cache, pkg, Some("already cached, skipping"));
                success += 1;
                continue;
            }
            match self.install(pkg).await {
                Ok(_) => {
                    success += 1;
                    cached_pkgs.insert(pkg.clone());
                }
                Err(e) => {
                    self.tui.print_line(
                        Label::Warn,
                        &format!("failed: {}", pkg),
                        Some(&e.to_string()),
                    );
                    failed.push(pkg.clone());
                }
            }
        }

        println!();
        self.tui.print_line(
            Label::Done,
            &format!("{}/{} packages cached", success, pkg_names.len()),
            None,
        );
        if !failed.is_empty() {
            self.tui.print_line(
                Label::Warn,
                "some packages failed",
                Some(&failed.join(", ")),
            );
        }
        Ok(())
    }
}

fn select_pypi_download_url(urls: &[serde_json::Value], pkg: &str) -> Result<String> {
    urls.iter()
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
        .ok_or_else(|| anyhow!("No downloadable file on PyPI for '{}'", pkg))
        .map(|s| s.to_string())
}

pub(crate) fn parse_pyproject_deps(content: &str) -> Result<Vec<String>> {
    let parsed: toml::Value =
        toml::from_str(content).map_err(|e| anyhow!("Failed to parse pyproject.toml: {}", e))?;

    let deps = parsed
        .get("project")
        .and_then(|p| p.get("dependencies"))
        .and_then(|d| d.as_array())
        .ok_or_else(|| anyhow!("No [project.dependencies] in pyproject.toml"))?;

    let pkg_names: Vec<String> = deps
        .iter()
        .filter_map(|d| d.as_str())
        .map(|d| {
            d.split(|c: char| !c.is_alphanumeric() && c != '-' && c != '_')
                .next()
                .unwrap_or(d)
                .to_lowercase()
        })
        .collect();

    Ok(pkg_names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pyproject_deps() {
        let content = r#"
[project]
name = "my-fastapi-app"
version = "0.1.0"
dependencies = [
    "fastapi>=0.110.0",
    "uvicorn[standard]>=0.28.0",
    "pydantic==2.6.4",
    "requests",
]
"#;
        let deps = parse_pyproject_deps(content).unwrap();
        assert_eq!(deps, vec!["fastapi", "uvicorn", "pydantic", "requests"]);
    }

    #[test]
    fn test_parse_pyproject_missing_deps() {
        let content = r#"
[project]
name = "empty-project"
"#;
        let res = parse_pyproject_deps(content);
        assert!(res.is_err());
    }
}
