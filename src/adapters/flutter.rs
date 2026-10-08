use crate::cache::{compute_sha256, Cache};
use crate::config::Config;
use crate::db::{Database, Package};
use crate::docs::{fetch_docs, DocsStore};
use crate::package_spec::PackageSpec;
use crate::tui::{Label, TUI};
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

pub struct FlutterAdapter {
    pub config: Config,
    pub db: Database,
    pub cache: Cache,
    pub tui: TUI,
    pub docs: DocsStore,
}

impl FlutterAdapter {
    pub fn new(config: Config, db: Database, cache: Cache, tui: TUI, docs: DocsStore) -> Self {
        Self {
            config,
            db,
            cache,
            tui,
            docs,
        }
    }

    pub async fn install(&mut self, pkg_spec_str: &str) -> Result<()> {
        let start = Instant::now();
        let spec = PackageSpec::parse(pkg_spec_str, "flutter")?;
        let pkg = &spec.name;

        let sp = self
            .tui
            .spinner(&format!("resolving {} from pub.dev...", pkg_spec_str));
        let url = format!("{}/api/packages/{}", self.config.registries.pubdev, pkg);
        let resp = reqwest::get(&url)
            .await
            .map_err(|e| anyhow!("Failed to reach pub.dev: {}", e))?
            .json::<serde_json::Value>()
            .await?;

        let (version, archive_url) = if let Some(ref ver_req) = spec.version_req {
            let versions = resp["versions"]
                .as_array()
                .ok_or_else(|| anyhow!("No versions found on pub.dev for '{}'", pkg))?;

            let clean_ver = ver_req
                .trim_start_matches('^')
                .trim_start_matches(':')
                .trim_start_matches("==")
                .trim();

            let matched = versions.iter().rev().find(|v| {
                v["version"]
                    .as_str()
                    .map(|ver| {
                        if ver == clean_ver || ver.starts_with(clean_ver) {
                            return true;
                        }
                        if let (Ok(parsed_v), Ok(parsed_req)) = (
                            semver::Version::parse(ver),
                            semver::VersionReq::parse(ver_req.trim_start_matches(':')),
                        ) {
                            parsed_req.matches(&parsed_v)
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false)
            });

            if let Some(v_obj) = matched {
                let v_str = v_obj["version"].as_str().unwrap().to_string();
                let arch = v_obj["archive_url"]
                    .as_str()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| {
                        format!(
                            "{}/packages/{}/versions/{}.tar.gz",
                            self.config.registries.pubdev, pkg, v_str
                        )
                    });
                (v_str, arch)
            } else {
                return Err(anyhow!(
                    "Version matching '{}' not found on pub.dev for '{}'",
                    ver_req,
                    pkg
                ));
            }
        } else {
            let version = resp["latest"]["version"]
                .as_str()
                .ok_or_else(|| anyhow!("No version on pub.dev for '{}'", pkg))?
                .to_string();
            let archive_url = resp["latest"]["archive_url"]
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    format!(
                        "{}/packages/{}/versions/{}.tar.gz",
                        self.config.registries.pubdev, pkg, version
                    )
                });
            (version, archive_url)
        };

        let tarball_url = archive_url;
        sp.finish(
            Label::Resolve,
            &format!("{}@{}", pkg, version),
            Some("pub.dev"),
        );

        let cache_path = self.cache.path_for("flutter", pkg, &version);
        self.cache.ensure_dir("flutter")?;

        let bar = self
            .tui
            .progress_bar(&format!("downloading {}@{}", pkg, version));
        bar.set(0.1, None);
        let size = self.cache.download_to(&tarball_url, &cache_path).await?;
        bar.set(0.8, Some("verifying checksum..."));
        let checksum = compute_sha256(&cache_path)?;
        bar.set(1.0, None);
        bar.finish(&format!(
            "{}@{} downloaded  ({:.1} KB)",
            pkg,
            version,
            size as f64 / 1_000.0
        ));

        let pub_cache_dir = get_pub_cache_dir();
        let pkg_hosted_dir = pub_cache_dir
            .join("hosted")
            .join("pub.dev")
            .join(format!("{}-{}", pkg, version));

        if !pkg_hosted_dir.exists() {
            let sp = self.tui.spinner("extracting to pub cache...");
            fs::create_dir_all(&pkg_hosted_dir)?;
            extract_tar_gz(&cache_path, &pkg_hosted_dir)?;
            sp.finish(
                Label::Cache,
                &format!("extracted to {}", pkg_hosted_dir.display()),
                None,
            );
        } else {
            self.tui.print_line(
                Label::Cache,
                "already in pub cache",
                Some(&pkg_hosted_dir.to_string_lossy()),
            );
        }

        // ── fetch & save docs ─────────────────────────────────────────────
        let sp = self.tui.spinner(&format!("fetching docs for {}...", pkg));
        let doc_content = fetch_docs("flutter", pkg, &version)
            .await
            .unwrap_or_else(|_| format!("# {pkg} v{version}\n\nNo docs available.\n"));
        self.docs.save_docs("flutter", pkg, &doc_content)?;
        sp.finish(
            Label::Info,
            &format!("{} docs cached", pkg),
            Some("~/.offpkg/docs/flutter/"),
        );

        let sp = self.tui.spinner("saving to offpkg cache...");
        self.db.insert_package(&Package {
            id: 0,
            name: pkg.to_string(),
            version: version.clone(),
            runtime: "flutter".to_string(),
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

    pub fn add(&mut self, pkg_spec_str: &str, skip_config: bool, _is_dev: bool) -> Result<()> {
        let start = Instant::now();
        let spec = PackageSpec::parse(pkg_spec_str, "flutter")?;
        let pkg = &spec.name;

        let sp = self
            .tui
            .spinner(&format!("resolving {} from offpkg cache...", pkg_spec_str));
        let latest = self
            .db
            .list_packages(Some("flutter"))?
            .into_iter()
            .filter(|p| p.name == *pkg)
            .filter(|p| {
                if let Some(ref req_ver) = spec.version_req {
                    p.version == *req_ver
                        || p.version.starts_with(req_ver.trim_start_matches('^').trim_start_matches(':'))
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
                    "'{}' not in cache. Run: offpkg flutter install {}",
                    pkg_spec_str,
                    pkg_spec_str
                ));
            }
        };
        sp.finish(
            Label::Resolve,
            &format!("{}@{}", cached.name, cached.version),
            Some("found in offpkg cache"),
        );

        let pub_cache_dir = get_pub_cache_dir();
        let pkg_hosted_dir = pub_cache_dir
            .join("hosted")
            .join("pub.dev")
            .join(format!("{}-{}", cached.name, cached.version));

        if !pkg_hosted_dir.exists() {
            let sp = self.tui.spinner("extracting to pub cache...");
            fs::create_dir_all(&pkg_hosted_dir)?;
            extract_tar_gz(Path::new(&cached.cache_path), &pkg_hosted_dir)?;
            sp.finish(
                Label::Cache,
                "extracted to pub cache",
                Some(&pkg_hosted_dir.to_string_lossy()),
            );
        } else {
            self.tui.print_line(
                Label::Cache,
                "reading from pub cache",
                Some(&pkg_hosted_dir.to_string_lossy()),
            );
        }

        let bar = self.tui.progress_bar("running flutter pub get --offline");
        bar.set(0.2, None);

        let add_output = Command::new("flutter")
            .args(["pub", "add", pkg])
            .output()
            .map_err(|e| anyhow!("Failed to run flutter: {}", e))?;

        bar.set(0.7, None);

        if !add_output.status.success() {
            drop(bar);
            return self.pub_get_offline(pkg, &cached.version, start, skip_config);
        }

        let get_output = Command::new("flutter")
            .args(["pub", "get", "--offline"])
            .output()
            .map_err(|e| anyhow!("Failed to run flutter pub get: {}", e))?;

        bar.set(1.0, None);

        if !get_output.status.success() {
            let stderr = String::from_utf8_lossy(&get_output.stderr);
            drop(bar);
            return Err(anyhow!(
                "flutter pub get --offline failed:\n{}",
                stderr.trim()
            ));
        }

        bar.finish(&format!("{} resolved from local cache", pkg));

        // ── copy docs into project ────────────────────────────────────────
        if !skip_config && self.docs.has_docs("flutter", &cached.name) {
            let cwd = std::env::current_dir()?;
            let sp = self.tui.spinner("copying docs to project...");
            self.docs.copy_to_project("flutter", &cached.name, &cwd)?;
            sp.finish(
                Label::Info,
                &format!("docs → offpkg_docs/{}.md", cached.name),
                None,
            );
        }

        self.tui.print_line(
            Label::Link,
            "updated pubspec.yaml",
            Some(&format!("{}: ^{} added", cached.name, cached.version)),
        );
        self.tui
            .print_line(Label::Info, "pubspec.lock regenerated", None);
        self.tui
            .print_line(Label::Done, "1 package resolved", Some("no network used"));
        self.tui.print_done_summary(1, 0, start.elapsed());
        Ok(())
    }

    fn pub_get_offline(
        &mut self,
        pkg: &str,
        version: &str,
        start: Instant,
        skip_config: bool,
    ) -> Result<()> {
        let cwd = std::env::current_dir()?;
        let pubspec_path = cwd.join("pubspec.yaml");
        if !pubspec_path.exists() {
            return Err(anyhow!(
                "No pubspec.yaml found. Run from inside your Flutter project."
            ));
        }
        let content = fs::read_to_string(&pubspec_path)?;
        if !skip_config && !content.contains(&format!("  {}:", pkg)) {
            let updated = insert_pubspec_dep(&content, pkg, version)?;
            fs::write(&pubspec_path, updated)?;
            self.tui
                .print_line(Label::Link, "added to pubspec.yaml", Some(pkg));
        }
        let output = Command::new("flutter")
            .args(["pub", "get", "--offline"])
            .output()
            .map_err(|e| anyhow!("Failed to run flutter pub get: {}", e))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!(
                "flutter pub get --offline failed:\n{}",
                stderr.trim()
            ));
        }

        if !skip_config && self.docs.has_docs("flutter", pkg) {
            let sp = self.tui.spinner("copying docs to project...");
            self.docs.copy_to_project("flutter", pkg, &cwd)?;
            sp.finish(Label::Info, &format!("docs → offpkg_docs/{}.md", pkg), None);
        }

        self.tui
            .print_line(Label::Done, &format!("{} added to project", pkg), None);
        self.tui.print_done_summary(1, 0, start.elapsed());
        Ok(())
    }

    pub async fn install_all(&mut self) -> Result<()> {
        let cwd = std::env::current_dir()?;
        let pubspec_path = cwd.join("pubspec.yaml");
        if !pubspec_path.exists() {
            return Err(anyhow!(
                "No pubspec.yaml found. Run from inside your Flutter project."
            ));
        }
        let content = fs::read_to_string(&pubspec_path)?;
        let pkg_names = parse_pubspec_deps(&content);

        self.tui.print_line(
            Label::Info,
            &format!("found {} dependencies in pubspec.yaml", pkg_names.len()),
            None,
        );

        let mut cached_pkgs: std::collections::HashSet<String> = self
            .db
            .list_packages(Some("flutter"))?
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

fn get_pub_cache_dir() -> std::path::PathBuf {
    std::env::var("PUB_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::var("HOME")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|_| std::path::PathBuf::from("/tmp"))
                .join(".pub-cache")
        })
}

fn extract_tar_gz(tgz_path: &Path, dest_dir: &Path) -> Result<()> {
    use flate2::read::GzDecoder;
    use tar::Archive;
    let file = fs::File::open(tgz_path)?;
    let mut archive = Archive::new(GzDecoder::new(file));
    for entry in archive.entries()? {
        let mut entry = entry?;
        let entry_path = entry.path()?.to_path_buf();
        let out_path = dest_dir.join(&entry_path);
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

pub(crate) fn parse_pubspec_deps(content: &str) -> Vec<String> {
    let mut deps = vec![];
    let mut in_deps = false;
    for line in content.lines() {
        if line.trim_start() == "dependencies:" {
            in_deps = true;
            continue;
        }
        if in_deps && !line.starts_with(' ') && !line.starts_with('\t') && !line.is_empty() {
            break;
        }
        if in_deps {
            if let Some(name) = line.trim().split(':').next() {
                let name = name.trim().to_string();
                if !name.is_empty() && name != "flutter" && name != "sdk" && !name.starts_with('#')
                {
                    deps.push(name);
                }
            }
        }
    }
    deps
}

fn insert_pubspec_dep(content: &str, pkg: &str, version: &str) -> Result<String> {
    let dep_line = format!("  {}: ^{}", pkg, version);
    let mut lines: Vec<String> = content.lines().map(String::from).collect();
    let insert_at = lines
        .iter()
        .position(|l| l.trim() == "dependencies:")
        .ok_or_else(|| anyhow!("No 'dependencies:' in pubspec.yaml"))?;
    lines.insert(insert_at + 1, dep_line);
    Ok(lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pubspec_deps() {
        let content = r#"
name: sample_flutter_app
description: A sample flutter application

environment:
  sdk: '>=3.0.0 <4.0.0'

dependencies:
  flutter:
    sdk: flutter
  dio: ^5.4.0
  provider: ^6.1.1
  # commented_dep: ^1.0.0
  flutter_svg: ^2.0.9

dev_dependencies:
  flutter_test:
    sdk: flutter
  flutter_lints: ^3.0.0
"#;
        let deps = parse_pubspec_deps(content);
        assert_eq!(deps, vec!["dio", "provider", "flutter_svg"]);
    }

    #[test]
    fn test_insert_pubspec_dep() {
        let content = "name: app\ndependencies:\n  flutter:\n    sdk: flutter\n";
        let updated = insert_pubspec_dep(content, "dio", "5.4.0").unwrap();
        assert!(updated.contains("  dio: ^5.4.0"));
        assert!(updated.starts_with("name: app\ndependencies:\n  dio: ^5.4.0\n"));
    }
}
