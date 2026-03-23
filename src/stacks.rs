use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use crate::config::Config;

// ── Stack definition ──────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StackFile {
    pub path: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stack {
    pub name: String,
    pub runtime: String,
    pub description: String,
    pub packages: Vec<String>,
    pub dev_packages: Vec<String>,
    pub files: Vec<StackFile>,
}

// ── Built-in stacks ───────────────────────────────────────────────────────────

pub fn builtin_stacks() -> Vec<Stack> {
    vec![
        Stack {
            name: "react-vite".into(),
            runtime: "bun".into(),
            description: "React + Vite + TypeScript — minimal starter".into(),
            packages: vec!["react".into(), "react-dom".into()],
            dev_packages: vec![
                "vite".into(), "@vitejs/plugin-react".into(),
                "typescript".into(), "@types/react".into(), "@types/react-dom".into(),
            ],
            files: vec![
                StackFile { path: "vite.config.ts".into(), content: r#"import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
export default defineConfig({ plugins: [react()] })"#.into() },
                StackFile { path: "tsconfig.json".into(), content: r#"{
  "compilerOptions": {
    "target": "ES2020",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "moduleResolution": "bundler",
    "jsx": "react-jsx",
    "strict": true
  },
  "include": ["src"]
}"#.into() },
                StackFile { path: "index.html".into(), content: r#"<!DOCTYPE html>
<html lang="en">
  <head><meta charset="UTF-8" /><title>App</title></head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>"#.into() },
                StackFile { path: "src/main.tsx".into(), content: r#"import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App'
createRoot(document.getElementById('root')!).render(
  <StrictMode><App /></StrictMode>
)"#.into() },
                StackFile { path: "src/App.tsx".into(), content: r#"export default function App() {
  return <h1>Hello from offpkg</h1>
}"#.into() },
            ],
        },
        Stack {
            name: "react-vite-full".into(),
            runtime: "bun".into(),
            description: "React + Vite + Zustand + TanStack Query + React Hook Form + Zod + Axios".into(),
            packages: vec![
                "react".into(), "react-dom".into(),
                "zustand".into(), "@tanstack/react-query".into(),
                "react-hook-form".into(), "@hookform/resolvers".into(),
                "zod".into(), "axios".into(),
            ],
            dev_packages: vec![
                "vite".into(), "@vitejs/plugin-react".into(),
                "typescript".into(), "@types/react".into(), "@types/react-dom".into(),
            ],
            files: vec![
                StackFile { path: "vite.config.ts".into(), content: "import { defineConfig } from 'vite'\nimport react from '@vitejs/plugin-react'\nexport default defineConfig({ plugins: [react()] })".into() },
                StackFile { path: "src/main.tsx".into(), content: r#"import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import App from './App'
const queryClient = new QueryClient()
createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}><App /></QueryClientProvider>
  </StrictMode>
)"#.into() },
                StackFile { path: "src/store.ts".into(), content: r#"import { create } from 'zustand'
interface AppState { count: number; increment: () => void }
export const useAppStore = create<AppState>((set) => ({
  count: 0,
  increment: () => set((s) => ({ count: s.count + 1 })),
}))"#.into() },
            ],
        },
        Stack {
            name: "hono-api".into(),
            runtime: "bun".into(),
            description: "Hono API + Prisma + Zod + Pino + PostgreSQL".into(),
            packages: vec!["hono".into(), "@prisma/client".into(), "zod".into(), "pino".into(), "pg".into(), "dotenv".into()],
            dev_packages: vec!["typescript".into(), "@types/node".into(), "prisma".into(), "pino-pretty".into(), "@types/pg".into()],
            files: vec![
                StackFile { path: "src/index.ts".into(), content: r#"import { Hono } from 'hono'
import { logger } from 'hono/logger'
const app = new Hono()
app.use('*', logger())
app.get('/', (c) => c.json({ message: 'Hello from offpkg' }))
export default { port: process.env.PORT || 3000, fetch: app.fetch }"#.into() },
                StackFile { path: ".env".into(), content: "DATABASE_URL=\"postgresql://user:password@localhost:5432/mydb\"\nPORT=3000".into() },
            ],
        },
        Stack {
            name: "fastapi".into(),
            runtime: "uv".into(),
            description: "FastAPI + SQLAlchemy + Alembic + Pydantic + Uvicorn".into(),
            packages: vec!["fastapi".into(), "uvicorn".into(), "sqlalchemy".into(), "asyncpg".into(), "alembic".into(), "pydantic-settings".into(), "python-dotenv".into(), "structlog".into()],
            dev_packages: vec![],
            files: vec![
                StackFile { path: "app/main.py".into(), content: "from fastapi import FastAPI\napp = FastAPI()\n\n@app.get(\"/\")\nasync def root():\n    return {\"message\": \"Hello from offpkg\"}\n".into() },
                StackFile { path: ".env".into(), content: "DATABASE_URL=postgresql+asyncpg://user:password@localhost:5432/mydb\nDEBUG=true".into() },
            ],
        },
        Stack {
            name: "flutter-riverpod".into(),
            runtime: "flutter".into(),
            description: "Flutter + Riverpod + Hooks + go_router + Dio + Logger".into(),
            packages: vec!["flutter_riverpod".into(), "hooks_riverpod".into(), "flutter_hooks".into(), "go_router".into(), "dio".into(), "logger".into()],
            dev_packages: vec![],
            files: vec![
                StackFile { path: "lib/main.dart".into(), content: r#"import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
void main() {
  runApp(const ProviderScope(child: MyApp()));
}
class MyApp extends StatelessWidget {
  const MyApp({super.key});
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'offpkg App',
      home: Scaffold(
        appBar: AppBar(title: const Text('Hello from offpkg')),
        body: const Center(child: Text('Stack ready')),
      ),
    );
  }
}"#.into() },
            ],
        },
    ]
}

// ── Interactive stack creator ─────────────────────────────────────────────────

const CYAN:  &str = "\x1b[38;2;0;212;224m";
const GREEN: &str = "\x1b[38;2;0;229;160m";
const AMBER: &str = "\x1b[38;2;245;166;35m";
const MUTED: &str = "\x1b[38;2;100;116;139m";
const BOLD:  &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

fn prompt(label: &str, hint: Option<&str>) -> Result<String> {
    if let Some(h) = hint {
        print!("  {}{}{} {} {} ", BOLD, CYAN, label, RESET, format!("{}({}){}", MUTED, h, RESET));
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

    // Accept number or name
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
    println!("  {}enter packages one per line, empty line when done{}", MUTED, RESET);

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

    // Stack name
    let name = prompt("stack name", Some("e.g. my-react-setup"))?;
    if name.is_empty() {
        return Err(anyhow!("Stack name cannot be empty"));
    }
    if name.contains(' ') {
        return Err(anyhow!("Stack name cannot contain spaces — use hyphens"));
    }

    // Runtime
    let runtime = prompt_select("runtime", &["bun", "uv", "flutter"])?;

    // Description
    let description = prompt("description", Some("short description of this stack"))?;
    let description = if description.is_empty() {
        format!("Custom {} stack", runtime)
    } else {
        description
    };

    println!();

    // Packages
    let packages = prompt_packages("packages")?;
    if packages.is_empty() {
        return Err(anyhow!("At least one package is required"));
    }

    println!();

    // Dev packages
    let has_dev = confirm("add dev dependencies?")?;
    let dev_packages = if has_dev {
        println!();
        prompt_packages("dev packages")?
    } else {
        vec![]
    };

    println!();

    // Summary
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
        files: vec![],
    };

    // Save to ~/.offpkg/stacks/<name>.toml
    fs::create_dir_all(custom_dir)?;
    let path = custom_dir.join(format!("{}.toml", name));
    let content = toml::to_string_pretty(&stack)
        .map_err(|e| anyhow!("Failed to serialize stack: {}", e))?;
    fs::write(&path, &content)?;

    println!();
    println!("  {}{}✓{} stack saved to {}{}{}", BOLD, GREEN, RESET, MUTED, path.display(), RESET);
    println!();
    println!("  {}next steps:{}", MUTED, RESET);
    println!("  {}→{} cache packages  {}offpkg stack install {}{}", CYAN, RESET, AMBER, name, RESET);
    println!("  {}→{} use in project  {}offpkg stack add {}{}", CYAN, RESET, AMBER, name, RESET);
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
        let mut stacks = builtin_stacks();
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
            if dest.exists() { continue; }
            fs::write(&dest, &file.content)
                .map_err(|e| anyhow!("Failed to write {:?}: {}", dest, e))?;
            created.push(file.path.clone());
        }
        Ok(created)
    }

    pub fn create_interactive(&self) -> Result<Stack> {
        interactive_create(&self.custom_dir())
    }

    pub fn delete(&self, name: &str) -> Result<()> {
        // only custom stacks can be deleted
        let path = self.custom_dir().join(format!("{}.toml", name));
        if !path.exists() {
            return Err(anyhow!(
                "'{}' is a built-in stack and cannot be deleted.", name
            ));
        }
        fs::remove_file(&path)?;
        Ok(())
    }
}