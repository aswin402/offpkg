use crate::config::Config;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct DocsStore {
    config: Config,
}

impl DocsStore {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// ~/.offpkg/docs/<runtime>/
    fn docs_dir(&self, runtime: &str) -> PathBuf {
        self.config.cache_path().join("docs").join(runtime)
    }

    /// ~/.offpkg/docs/<runtime>/<safe_pkg>.md  — the global master copy you edit
    /// Scoped packages like @vitejs/plugin-react become __vitejs__plugin-react.md
    pub fn global_doc_path(&self, runtime: &str, pkg: &str) -> PathBuf {
        let safe = pkg.replace('@', "__").replace('/', "__");
        self.docs_dir(runtime).join(format!("{}.md", safe))
    }

    pub fn has_docs(&self, runtime: &str, pkg: &str) -> bool {
        self.global_doc_path(runtime, pkg).exists()
    }

    /// Save docs to global cache (called during install).
    /// Always writes — install should always refresh the base doc.
    /// User edits are preserved because install only runs when explicitly called.
    pub fn save_docs(&self, runtime: &str, pkg: &str, content: &str) -> Result<()> {
        let dir = self.docs_dir(runtime);
        fs::create_dir_all(&dir)
            .map_err(|e| anyhow::anyhow!("Failed to create docs dir {:?}: {}", dir, e))?;
        let path = self.global_doc_path(runtime, pkg);
        fs::write(&path, content)
            .map_err(|e| anyhow::anyhow!("Failed to write doc {:?}: {}", path, e))?;
        Ok(())
    }

    /// Force overwrite global doc — called only by `offpkg docs reset`
    pub fn reset_global_doc(&self, runtime: &str, pkg: &str, content: &str) -> Result<()> {
        fs::create_dir_all(self.docs_dir(runtime))?;
        fs::write(self.global_doc_path(runtime, pkg), content)?;
        Ok(())
    }

    /// Copy global doc (with user edits) into project as offpkg_docs/offpkg_<pkg>.md
    /// Always copies — project file is just a read copy, master lives in ~/.offpkg/docs/
    pub fn copy_to_project(&self, runtime: &str, pkg: &str, project_dir: &Path) -> Result<PathBuf> {
        let global_path = self.global_doc_path(runtime, pkg);

        if !global_path.exists() {
            return Err(anyhow!(
                "No docs found for '{}'. Run: offpkg {} install {}",
                pkg,
                runtime,
                pkg
            ));
        }

        let project_docs_dir = project_dir.join("offpkg_docs");
        fs::create_dir_all(&project_docs_dir)?;

        // File is named offpkg_<safe_pkg>.md in the project
        // Scoped packages: @vitejs/plugin-react -> offpkg___vitejs__plugin-react.md
        let safe = pkg.replace('@', "__").replace('/', "__");
        let dest = project_docs_dir.join(format!("offpkg_{}.md", safe));
        fs::copy(&global_path, &dest)?;

        Ok(dest)
    }

    /// Read global doc content
    pub fn read_docs(&self, runtime: &str, pkg: &str) -> Result<String> {
        let path = self.global_doc_path(runtime, pkg);
        if !path.exists() {
            return Err(anyhow!(
                "No docs cached for '{}' ({}).\nRun: offpkg {} install {}",
                pkg,
                runtime,
                runtime,
                pkg
            ));
        }
        Ok(fs::read_to_string(path)?)
    }

    /// Open the GLOBAL doc in editor so edits apply to all future project copies
    pub fn open_in_editor(&self, runtime: &str, pkg: &str) -> Result<()> {
        let doc_path = self.global_doc_path(runtime, pkg);

        if !doc_path.exists() {
            return Err(anyhow!(
                "No docs found for '{}'. Run: offpkg {} install {} first.",
                pkg,
                runtime,
                pkg
            ));
        }

        let editor = std::env::var("EDITOR").unwrap_or_else(|_| {
            for e in &["nvim", "vim", "nano", "code", "gedit"] {
                if which_available(e) {
                    return e.to_string();
                }
            }
            "nano".to_string()
        });

        println!(
            "opening ~/.offpkg/docs/{}/{}.md in {}...",
            runtime, pkg, editor
        );

        let status = std::process::Command::new(&editor)
            .arg(&doc_path)
            .status()
            .map_err(|e| anyhow!("Failed to open '{}': {}", editor, e))?;

        if !status.success() {
            return Err(anyhow!("Editor exited with error"));
        }

        Ok(())
    }
}

fn which_available(name: &str) -> bool {
    std::process::Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Fetch docs from registry README. Falls back to template.
pub async fn fetch_docs(runtime: &str, pkg: &str, version: &str) -> Result<String> {
    let result = match runtime {
        "flutter" => fetch_pubdev_docs(pkg, version).await,
        "bun" => fetch_npm_docs(pkg, version).await,
        "uv" => fetch_pypi_docs(pkg, version).await,
        _ => Err(anyhow!("Unknown runtime")),
    };
    match result {
        Ok(c) if !c.trim().is_empty() => Ok(c),
        _ => Ok(generate_template(runtime, pkg, version)),
    }
}

async fn fetch_pubdev_docs(pkg: &str, version: &str) -> Result<String> {
    let resp = reqwest::get(&format!("https://pub.dev/api/packages/{}", pkg))
        .await?
        .json::<serde_json::Value>()
        .await?;

    let description = resp["latest"]["pubspec"]["description"]
        .as_str()
        .unwrap_or("")
        .to_string();
    let homepage = resp["latest"]["pubspec"]["homepage"]
        .as_str()
        .or_else(|| resp["latest"]["pubspec"]["repository"].as_str())
        .unwrap_or("")
        .to_string();
    let likes = resp["likes"].as_i64().unwrap_or(0);
    let points = resp["pub_points"].as_i64().unwrap_or(0);

    let readme_resp = reqwest::get(&format!(
        "https://pub.dev/api/packages/{}/versions/{}/readme",
        pkg, version
    ))
    .await;

    let readme = if let Ok(r) = readme_resp {
        r.json::<serde_json::Value>()
            .await
            .ok()
            .and_then(|j| j["content"].as_str().map(|s| s.to_string()))
            .unwrap_or_default()
    } else {
        String::new()
    };

    let header = format!(
        "# {pkg} — offpkg docs\n\
         > **Version**: {version} · **Runtime**: flutter · **pub.dev**: https://pub.dev/packages/{pkg}  \n\
         > **Likes**: {likes} · **Points**: {points}  \n\
         \n\
         {description}\n\
         **Homepage**: {homepage}\n\n\
         ---\n\n\
         > ✏️ Edit this file freely — it lives in ~/.offpkg/docs/flutter/{pkg}.md\n\
         > Every project you add {pkg} to will get YOUR edited version.\n\
         > To regenerate from original: `offpkg docs reset {pkg} --runtime flutter`\n\n\
         ## My Notes\n\n\
         <!-- Add your own notes, snippets, team conventions here -->\n\n\
         ---\n\n"
    );

    Ok(format!(
        "{}{}",
        header,
        if readme.is_empty() {
            generate_flutter_usage(pkg, version)
        } else {
            readme
        }
    ))
}

async fn fetch_npm_docs(pkg: &str, version: &str) -> Result<String> {
    let resp = reqwest::get(&format!("https://registry.npmjs.org/{}", pkg))
        .await?
        .json::<serde_json::Value>()
        .await?;

    let description = resp["description"].as_str().unwrap_or("").to_string();
    let homepage = resp["homepage"].as_str().unwrap_or("").to_string();
    let readme = resp["readme"].as_str().unwrap_or("").to_string();

    let header = format!(
        "# {pkg} — offpkg docs\n\
         > **Version**: {version} · **Runtime**: bun · **npm**: https://www.npmjs.com/package/{pkg}  \n\
         \n\
         {description}\n\
         **Homepage**: {homepage}\n\n\
         ---\n\n\
         > ✏️ Edit this file freely — it lives in ~/.offpkg/docs/bun/{pkg}.md\n\
         > Every project you add {pkg} to will get YOUR edited version.\n\
         > To regenerate from original: `offpkg docs reset {pkg} --runtime bun`\n\n\
         ## My Notes\n\n\
         <!-- Add your own notes, snippets, team conventions here -->\n\n\
         ---\n\n"
    );

    Ok(format!(
        "{}{}",
        header,
        if readme.is_empty() {
            generate_bun_usage(pkg, version)
        } else {
            readme
        }
    ))
}

async fn fetch_pypi_docs(pkg: &str, version: &str) -> Result<String> {
    let resp = reqwest::get(&format!("https://pypi.org/pypi/{}/json", pkg))
        .await?
        .json::<serde_json::Value>()
        .await?;

    let description = resp["info"]["summary"].as_str().unwrap_or("").to_string();
    let homepage = resp["info"]["home_page"].as_str().unwrap_or("").to_string();
    let docs_url = resp["info"]["docs_url"].as_str().unwrap_or("").to_string();
    let long_desc = resp["info"]["description"]
        .as_str()
        .unwrap_or("")
        .to_string();

    let header = format!(
        "# {pkg} — offpkg docs\n\
         > **Version**: {version} · **Runtime**: uv · **PyPI**: https://pypi.org/project/{pkg}  \n\
         \n\
         {description}\n\
         **Homepage**: {homepage} · **Docs**: {docs_url}\n\n\
         ---\n\n\
         > ✏️ Edit this file freely — it lives in ~/.offpkg/docs/uv/{pkg}.md\n\
         > Every project you add {pkg} to will get YOUR edited version.\n\
         > To regenerate from original: `offpkg docs reset {pkg} --runtime uv`\n\n\
         ## My Notes\n\n\
         <!-- Add your own notes, snippets, team conventions here -->\n\n\
         ---\n\n"
    );

    let body = if long_desc.len() > 100 {
        long_desc
    } else {
        generate_uv_usage(pkg, version)
    };
    Ok(format!("{}{}", header, body))
}

fn generate_template(runtime: &str, pkg: &str, version: &str) -> String {
    match runtime {
        "flutter" => generate_flutter_usage(pkg, version),
        "bun" => generate_bun_usage(pkg, version),
        "uv" => generate_uv_usage(pkg, version),
        _ => format!("# {pkg} v{version}\n\nNo docs available.\n"),
    }
}

fn generate_flutter_usage(pkg: &str, version: &str) -> String {
    format!(
        "## Installation\n\n\
         ```yaml\n\
         dependencies:\n  {pkg}: ^{version}\n\
         ```\n\n\
         ## Import\n\n\
         ```dart\n\
         import 'package:{pkg}/{pkg}.dart';\n\
         ```\n\n\
         ## Quick Start\n\n\
         ```dart\n\
         // Add your usage example here\n\
         ```\n\n\
         ## Links\n\n\
         - pub.dev: https://pub.dev/packages/{pkg}\n\
         - API docs: https://pub.dev/documentation/{pkg}/latest/\n\
         - Changelog: https://pub.dev/packages/{pkg}/changelog\n"
    )
}

fn generate_bun_usage(pkg: &str, version: &str) -> String {
    format!(
        "## Installation\n\n\
         ```json\n{{ \"dependencies\": {{ \"{pkg}\": \"^{version}\" }} }}\n```\n\n\
         ## Import\n\n\
         ```typescript\nimport ... from '{pkg}';\n```\n\n\
         ## Quick Start\n\n\
         ```typescript\n// Add your usage example here\n```\n\n\
         ## Links\n\n\
         - npm: https://www.npmjs.com/package/{pkg}\n"
    )
}

fn generate_uv_usage(pkg: &str, version: &str) -> String {
    let import_name = pkg.replace('-', "_");
    format!(
        "## Installation\n\n\
         ```toml\n[project]\ndependencies = [\"{pkg}>={version}\"]\n```\n\n\
         ## Import\n\n\
         ```python\nimport {import_name}\n```\n\n\
         ## Quick Start\n\n\
         ```python\n# Add your usage example here\n```\n\n\
         ## Links\n\n\
         - PyPI: https://pypi.org/project/{pkg}\n"
    )
}
