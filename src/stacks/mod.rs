use crate::config::Config;
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
            "  {}{}{} {} {} ",
            BOLD,
            CYAN,
            label,
            RESET,
            format!("{}({}){}", MUTED, h, RESET)
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

// ── StackStore ────────────────────────────────────────────────────────────────

pub struct StackStore {
    _config: Config,
}

impl StackStore {
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    pub fn custom_dir(&self) -> PathBuf {
        let home = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/tmp"));
        home.join(".offpkg").join("stacks")
    }

    pub fn all_stacks(&self) -> Vec<Stack> {
        let mut stacks = builtin::builtin_stacks();
        let dir = self.custom_dir();
        if dir.exists() {
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(stack) = toml::from_str::<Stack>(&content) {
                                stacks.push(stack);
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

    pub fn write_files(&self, stack: &Stack, project_dir: &Path) -> Result<Vec<String>> {
        let mut created = vec![];
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
                fs::write(&dest, &file.content)
                    .map_err(|e| anyhow!("Failed to write {:?}: {}", dest, e))?;
            }
            created.push(file.path.clone());
        }
        Ok(created)
    }

    pub fn create_interactive(&self) -> Result<Stack> {
        interactive_create(&self.custom_dir())
    }

    pub fn delete(&self, name: &str) -> Result<()> {
        let path = self.custom_dir().join(format!("{}.toml", name));
        if !path.exists() {
            return Err(anyhow!(
                "'{}' is a built-in stack and cannot be deleted.",
                name
            ));
        }
        fs::remove_file(&path)?;
        Ok(())
    }
}
