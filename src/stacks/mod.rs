use crate::config::Config;
use crate::db::Database;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub mod builtin;

// ── Stack definition ──────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StackFile {
    pub path: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binary_content: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stack {
    pub name: String,
    pub runtime: String,
    pub description: String,
    pub packages: Vec<String>,
    pub dev_packages: Vec<String>,
    #[serde(default)]
    pub transitive_packages: Vec<String>,
    pub files: Vec<StackFile>,
}

// ── Interactive stack creator ─────────────────────────────────────────────────

const CYAN: &str = "\x1b[38;2;0;212;224m";
const GREEN: &str = "\x1b[38;2;0;229;160m";
const AMBER: &str = "\x1b[38;2;245;166;35m";
const MUTED: &str = "\x1b[38;2;100;116;139m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

fn prompt(label: &str, hint: Option<&str>) -> Result<String> {
    if let Some(h) = hint {
        print!(
            "  {}{}{} {} {}({}){} ",
            BOLD, CYAN, label, RESET, MUTED, h, RESET
        );
    } else {
        print!("  {}{}{} {} ", BOLD, CYAN, label, RESET);
    }
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_select(label: &str, options: &[&str]) -> Result<String> {
    println!("  {}{}{}{}", BOLD, CYAN, label, RESET);
    for (i, opt) in options.iter().enumerate() {
        println!("    {}[{}]{} {}", MUTED, i + 1, RESET, opt);
    }
    print!("  {}→{} ", CYAN, RESET);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();

    if let Ok(n) = trimmed.parse::<usize>() {
        if n >= 1 && n <= options.len() {
            return Ok(options[n - 1].to_string());
        }
    }
    if options.contains(&trimmed) {
        return Ok(trimmed.to_string());
    }
    Err(anyhow!("Invalid selection: {}", trimmed))
}

fn prompt_packages(label: &str) -> Result<Vec<String>> {
    println!("  {}{}{}{}", BOLD, CYAN, label, RESET);
    println!(
        "  {}enter packages one per line, empty line when done{}",
        MUTED, RESET
    );

    let mut packages = vec![];
    loop {
        print!("  {}+{} ", GREEN, RESET);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let pkg = input.trim().to_string();
        if pkg.is_empty() {
            break;
        }
        packages.push(pkg);
    }
    Ok(packages)
}

fn confirm(label: &str) -> Result<bool> {
    print!("  {}{}{} {}[y/n]{} ", BOLD, CYAN, label, MUTED, RESET);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"))
}

pub fn interactive_create(custom_dir: &Path) -> Result<Stack> {
    println!();
    println!("  {}{}offpkg stack creator{}", BOLD, CYAN, RESET);
    println!("  {}────────────────────────{}", MUTED, RESET);
    println!();

    let name = prompt("stack name", Some("e.g. my-react-setup"))?;
    if name.is_empty() {
        return Err(anyhow!("Stack name cannot be empty"));
    }
    if name.contains(' ') {
        return Err(anyhow!("Stack name cannot contain spaces — use hyphens"));
    }

    let runtime = prompt_select("runtime", &["bun", "uv", "flutter"])?;

    let description = prompt("description", Some("short description of this stack"))?;
    let description = if description.is_empty() {
        format!("Custom {} stack", runtime)
    } else {
        description
    };

    println!();

    let packages = prompt_packages("packages")?;
    if packages.is_empty() {
        return Err(anyhow!("At least one package is required"));
    }

    println!();

    let has_dev = confirm("add dev dependencies?")?;
    let dev_packages = if has_dev {
        println!();
        prompt_packages("dev packages")?
    } else {
        vec![]
    };

    println!();

    println!("  {}{}stack summary{}", BOLD, CYAN, RESET);
    println!("  {}────────────────────────{}", MUTED, RESET);
    println!("  {}name{}       {}", MUTED, RESET, name);
    println!("  {}runtime{}    {}", MUTED, RESET, runtime);
    println!("  {}packages{}   {}", MUTED, RESET, packages.join(", "));
    if !dev_packages.is_empty() {
        println!("  {}dev{}        {}", MUTED, RESET, dev_packages.join(", "));
    }
    println!();

    let confirmed = confirm("create this stack?")?;
    if !confirmed {
        return Err(anyhow!("Stack creation cancelled"));
    }

    let stack = Stack {
        name: name.clone(),
        runtime: runtime.clone(),
        description,
        packages,
        dev_packages,
        transitive_packages: vec![],
        files: vec![],
    };

    fs::create_dir_all(custom_dir)?;
    let path = custom_dir.join(format!("{}.toml", name));
    let content =
        toml::to_string_pretty(&stack).map_err(|e| anyhow!("Failed to serialize stack: {}", e))?;
    fs::write(&path, &content)?;

    println!();
    println!(
        "  {}{}✓{} stack saved to {}{}{}",
        BOLD,
        GREEN,
        RESET,
        MUTED,
        path.display(),
        RESET
    );
    println!();
    println!("  {}next steps:{}", MUTED, RESET);
    println!(
        "  {}→{} cache packages  {}offpkg stack install {}{}",
        CYAN, RESET, AMBER, name, RESET
    );
    println!(
        "  {}→{} use in project  {}offpkg stack add {}{}",
        CYAN, RESET, AMBER, name, RESET
    );
    println!();

    Ok(stack)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StackManifest {
    pub name: String,
    pub runtime: String,
    pub description: String,
    #[serde(default)]
    pub packages: Vec<String>,
    #[serde(default)]
    pub dev_packages: Vec<String>,
    #[serde(default)]
    pub transitive_packages: Vec<String>,
}

// ── StackStore ────────────────────────────────────────────────────────────────

pub struct StackStore {
    config: Config,
}

impl StackStore {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn custom_dir(&self) -> PathBuf {
        self.config.custom_stacks_dir()
    }

    pub fn templates_dir(&self) -> PathBuf {
        self.config.templates_dir()
    }

    pub fn all_stacks(&self) -> Vec<Stack> {
        let mut stacks = builtin::builtin_stacks();

        // 1. Directory-based templates from ~/.offpkg/templates/<name>/
        let t_dir = self.templates_dir();
        if t_dir.exists() {
            if let Ok(entries) = fs::read_dir(&t_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Ok(stack) = load_directory_stack(&path) {
                            if let Some(idx) = stacks.iter().position(|s| s.name == stack.name) {
                                stacks[idx] = stack;
                            } else {
                                stacks.push(stack);
                            }
                        }
                    }
                }
            }
        }

        // 2. Legacy custom .toml stacks from ~/.offpkg/stacks/<name>.toml
        let dir = self.custom_dir();
        if dir.exists() {
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(stack) = toml::from_str::<Stack>(&content) {
                                if let Some(idx) = stacks.iter().position(|s| s.name == stack.name) {
                                    stacks[idx] = stack;
                                } else {
                                    stacks.push(stack);
                                }
                            }
                        }
                    }
                }
            }
        }

        stacks
    }

    pub fn find(&self, name: &str) -> Option<Stack> {
        self.all_stacks().into_iter().find(|s| s.name == name)
    }

    pub fn template_path(&self, name: &str) -> Option<PathBuf> {
        let dir_path = self.templates_dir().join(name);
        if dir_path.exists() {
            return Some(dir_path);
        }
        let toml_path = self.custom_dir().join(format!("{}.toml", name));
        if toml_path.exists() {
            return Some(toml_path);
        }
        None
    }

    pub fn write_files_with_db(
        &self,
        stack: &Stack,
        project_dir: &Path,
        db: Option<&Database>,
    ) -> Result<Vec<String>> {
        let mut created = vec![];

        let mut cached_versions: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        if let Some(db_ref) = db {
            if let Ok(pkgs) = db_ref.list_packages(Some(&stack.runtime)) {
                for p in pkgs {
                    cached_versions.insert(p.name, p.version);
                }
            }
        }

        for file in &stack.files {
            let dest = project_dir.join(&file.path);
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            if dest.exists() {
                continue;
            }
            if let Some(binary) = &file.binary_content {
                fs::write(&dest, binary)
                    .map_err(|e| anyhow!("Failed to write binary {:?}: {}", dest, e))?;
            } else {
                let mut content = file.content.clone();

                // Dynamic placeholder replacement: {{<pkg>.version}} or {{<pkg>}}
                for (pkg_name, ver) in &cached_versions {
                    let p1 = ["{{", pkg_name, ".version}}"].concat();
                    let p2 = ["{{", pkg_name, "}}"].concat();
                    if content.contains(&p1) {
                        content = content.replace(&p1, ver);
                    }
                    if content.contains(&p2) {
                        content = content.replace(&p2, ver);
                    }
                }

                // If package.json, dynamically align dependency versions with cache if found
                if file.path == "package.json" && !cached_versions.is_empty() {
                    if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&content) {
                        let mut modified = false;
                        if let Some(deps) = json.get_mut("dependencies").and_then(|d| d.as_object_mut()) {
                            for (pkg, ver_val) in deps.iter_mut() {
                                if let Some(cached_v) = cached_versions.get(pkg) {
                                    *ver_val = serde_json::Value::String(format!("^{}", cached_v));
                                    modified = true;
                                }
                            }
                        }
                        if let Some(dev_deps) = json.get_mut("devDependencies").and_then(|d| d.as_object_mut()) {
                            for (pkg, ver_val) in dev_deps.iter_mut() {
                                if let Some(cached_v) = cached_versions.get(pkg) {
                                    *ver_val = serde_json::Value::String(format!("^{}", cached_v));
                                    modified = true;
                                }
                            }
                        }
                        if modified {
                            if let Ok(pretty) = serde_json::to_string_pretty(&json) {
                                content = pretty;
                            }
                        }
                    }
                }

                fs::write(&dest, &content)
                    .map_err(|e| anyhow!("Failed to write {:?}: {}", dest, e))?;
            }
            created.push(file.path.clone());
        }
        Ok(created)
    }

    pub fn write_files(&self, stack: &Stack, project_dir: &Path) -> Result<Vec<String>> {
        self.write_files_with_db(stack, project_dir, None)
    }

    pub fn save_from_project(
        &self,
        name: &str,
        custom_desc: Option<&str>,
        project_dir: &Path,
    ) -> Result<PathBuf> {
        let name = name.trim().to_lowercase();
        if name.is_empty() || name.contains(' ') {
            return Err(anyhow!("Template name must be non-empty and cannot contain spaces"));
        }

        let mut runtime = "bun".to_string();
        let mut packages = vec![];
        let mut dev_packages = vec![];

        let pkg_json_path = project_dir.join("package.json");
        let pyproject_path = project_dir.join("pyproject.toml");
        let pubspec_path = project_dir.join("pubspec.yaml");

        if pkg_json_path.exists() {
            runtime = "bun".to_string();
            if let Ok(content) = fs::read_to_string(&pkg_json_path) {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(deps) = v.get("dependencies").and_then(|d| d.as_object()) {
                        packages = deps.keys().cloned().collect();
                    }
                    if let Some(dev) = v.get("devDependencies").and_then(|d| d.as_object()) {
                        dev_packages = dev.keys().cloned().collect();
                    }
                }
            }
        } else if pyproject_path.exists() {
            runtime = "uv".to_string();
            if let Ok(content) = fs::read_to_string(&pyproject_path) {
                if let Ok(deps) = crate::adapters::uv::parse_pyproject_deps(&content) {
                    packages = deps;
                }
            }
        } else if pubspec_path.exists() {
            runtime = "flutter".to_string();
            if let Ok(content) = fs::read_to_string(&pubspec_path) {
                packages = crate::adapters::flutter::parse_pubspec_deps(&content);
            }
        }

        let description = custom_desc
            .map(String::from)
            .unwrap_or_else(|| format!("Custom {} template: {}", runtime, name));

        let target_dir = self.templates_dir().join(&name);
        if target_dir.exists() {
            return Err(anyhow!(
                "Template '{}' already exists at {}. Delete it first to overwrite.",
                name,
                target_dir.display()
            ));
        }

        fs::create_dir_all(&target_dir)?;

        let manifest = StackManifest {
            name: name.clone(),
            runtime,
            description,
            packages,
            dev_packages,
            transitive_packages: vec![],
        };
        let manifest_content = toml::to_string_pretty(&manifest)
            .map_err(|e| anyhow!("Failed to serialize manifest: {}", e))?;
        fs::write(target_dir.join("stack.toml"), manifest_content)?;

        copy_template_files(project_dir, &target_dir)?;

        Ok(target_dir)
    }

    pub fn create_interactive(&self) -> Result<Stack> {
        interactive_create(&self.custom_dir())
    }

    pub fn delete(&self, name: &str) -> Result<()> {
        let toml_path = self.custom_dir().join(format!("{}.toml", name));
        if toml_path.exists() {
            fs::remove_file(&toml_path)?;
            return Ok(());
        }

        let dir_path = self.templates_dir().join(name);
        if dir_path.exists() {
            fs::remove_dir_all(&dir_path)?;
            return Ok(());
        }

        Err(anyhow!(
            "'{}' is a built-in stack and cannot be deleted.",
            name
        ))
    }
}

fn load_directory_stack(dir: &Path) -> Result<Stack> {
    let manifest_path = if dir.join("stack.toml").exists() {
        dir.join("stack.toml")
    } else if dir.join("template.toml").exists() {
        dir.join("template.toml")
    } else if dir.join("offpkg.toml").exists() {
        dir.join("offpkg.toml")
    } else {
        return Err(anyhow!("No stack.toml or template.toml found in {:?}", dir));
    };

    let manifest_content = fs::read_to_string(&manifest_path)?;
    let manifest: StackManifest = toml::from_str(&manifest_content)?;

    let mut files = vec![];
    collect_files_recursive(dir, dir, &manifest_path, &mut files)?;

    Ok(Stack {
        name: manifest.name,
        runtime: manifest.runtime,
        description: manifest.description,
        packages: manifest.packages,
        dev_packages: manifest.dev_packages,
        transitive_packages: manifest.transitive_packages,
        files,
    })
}

fn collect_files_recursive(
    root: &Path,
    current: &Path,
    manifest_path: &Path,
    files: &mut Vec<StackFile>,
) -> Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if file_name.starts_with('.') && file_name != ".gitignore" && file_name != ".env.example" {
            continue;
        }
        if matches!(
            file_name,
            "node_modules"
                | ".venv"
                | "venv"
                | "target"
                | "dist"
                | "build"
                | ".git"
                | ".next"
                | "offpkg_docs"
                | ".turbo"
                | ".cache"
        ) {
            continue;
        }

        if path == *manifest_path {
            continue;
        }

        if path.is_dir() {
            collect_files_recursive(root, &path, manifest_path, files)?;
        } else if path.is_file() {
            let rel_path = path
                .strip_prefix(root)?
                .to_string_lossy()
                .replace('\\', "/");

            let bytes = fs::read(&path)?;
            match String::from_utf8(bytes) {
                Ok(content) => {
                    files.push(StackFile {
                        path: rel_path,
                        content,
                        binary_content: None,
                    });
                }
                Err(err) => {
                    files.push(StackFile {
                        path: rel_path,
                        content: String::new(),
                        binary_content: Some(err.into_bytes()),
                    });
                }
            }
        }
    }
    Ok(())
}

fn copy_template_files(src: &Path, dest: &Path) -> Result<()> {
    let canonical_dest = dest.canonicalize().ok();

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if file_name.starts_with('.') && file_name != ".gitignore" && file_name != ".env.example" {
            continue;
        }
        if matches!(
            file_name,
            "node_modules"
                | ".venv"
                | "venv"
                | "target"
                | "dist"
                | "build"
                | ".git"
                | ".next"
                | "offpkg_docs"
                | ".turbo"
                | ".cache"
                | "stack.toml"
                | "template.toml"
                | "offpkg.toml"
        ) {
            continue;
        }

        if let (Some(ref c_dest), Ok(c_path)) = (&canonical_dest, path.canonicalize()) {
            if c_dest.starts_with(&c_path) || c_path == **c_dest {
                continue;
            }
        }

        let target_path = dest.join(file_name);
        if path.is_dir() {
            fs::create_dir_all(&target_path)?;
            copy_template_files(&path, &target_path)?;
        } else if path.is_file() {
            fs::copy(&path, &target_path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_directory_stack() {
        let temp_dir = std::env::temp_dir().join(format!("offpkg_test_tpl_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();

        let manifest = r#"
name = "test-tpl"
runtime = "bun"
description = "A test template"
packages = ["react", "react-dom"]
dev_packages = ["vite"]
"#;
        fs::write(temp_dir.join("stack.toml"), manifest).unwrap();
        fs::write(temp_dir.join("index.html"), "<h1>Hello</h1>").unwrap();

        let sub = temp_dir.join("src");
        fs::create_dir_all(&sub).unwrap();
        fs::write(sub.join("main.ts"), "console.log('hi');").unwrap();

        let stack = load_directory_stack(&temp_dir).unwrap();
        assert_eq!(stack.name, "test-tpl");
        assert_eq!(stack.runtime, "bun");
        assert_eq!(stack.packages, vec!["react", "react-dom"]);
        assert_eq!(stack.dev_packages, vec!["vite"]);
        assert_eq!(stack.files.len(), 2);

        let paths: Vec<String> = stack.files.into_iter().map(|f| f.path).collect();
        assert!(paths.contains(&"index.html".to_string()));
        assert!(paths.contains(&"src/main.ts".to_string()));

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_dynamic_version_interpolation() {
        let store = StackStore::new(Config::default());
        let temp_dest = std::env::temp_dir().join(format!("offpkg_test_proj_{}", std::process::id()));
        fs::create_dir_all(&temp_dest).unwrap();

        let stack = Stack {
            name: "test-dynamic".into(),
            runtime: "bun".into(),
            description: "Dynamic test".into(),
            packages: vec!["react".into()],
            dev_packages: vec![],
            transitive_packages: vec![],
            files: vec![
                StackFile {
                    path: "config.txt".into(),
                    content: "React version is {{react.version}}".into(),
                    binary_content: None,
                },
                StackFile {
                    path: "package.json".into(),
                    content: r#"{
  "name": "test-app",
  "dependencies": {
    "react": "^0.0.0"
  }
}"#.into(),
                    binary_content: None,
                },
            ],
        };

        // Create temporary DB and insert a package
        let mut test_config = Config::default();
        test_config.cache.path = temp_dest.to_string_lossy().to_string();
        let db = Database::open(&test_config).unwrap();
        db.insert_package(&crate::db::Package {
            id: 0,
            name: "react".to_string(),
            version: "19.2.4".to_string(),
            runtime: "bun".to_string(),
            cache_path: "/tmp/react.tgz".to_string(),
            checksum: "abc".to_string(),
            size_bytes: Some(1234),
            cached_at: "now".to_string(),
        }).unwrap();

        let created = store.write_files_with_db(&stack, &temp_dest, Some(&db)).unwrap();
        assert_eq!(created.len(), 2);

        let cfg = fs::read_to_string(temp_dest.join("config.txt")).unwrap();
        assert_eq!(cfg, "React version is 19.2.4");

        let pkg_json = fs::read_to_string(temp_dest.join("package.json")).unwrap();
        assert!(pkg_json.contains("\"react\": \"^19.2.4\""));

        fs::remove_dir_all(&temp_dest).ok();
    }
}
