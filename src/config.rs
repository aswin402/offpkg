use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub cache: CacheConfig,
    pub network: NetworkConfig,
    pub runtimes: RuntimesConfig,
    #[serde(default)]
    pub registries: RegistriesConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CacheConfig {
    pub path: String,
    pub max_size_gb: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkConfig {
    pub timeout_secs: u64,
    pub retries: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RuntimesConfig {
    pub bun: String,
    pub uv: String,
    pub flutter: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegistriesConfig {
    pub npm: String,
    pub pypi: String,
    pub pubdev: String,
    pub github_repo: String,
}

fn default_npm_registry() -> String {
    "https://registry.npmjs.org".to_string()
}
fn default_pypi_registry() -> String {
    "https://pypi.org".to_string()
}
fn default_pubdev_registry() -> String {
    "https://pub.dev".to_string()
}
fn default_github_repo() -> String {
    "aswin402/offpkg".to_string()
}

impl Default for RegistriesConfig {
    fn default() -> Self {
        Self {
            npm: default_npm_registry(),
            pypi: default_pypi_registry(),
            pubdev: default_pubdev_registry(),
            github_repo: default_github_repo(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let home = home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        Self {
            cache: CacheConfig {
                path: home.join(".offpkg/cache").to_string_lossy().to_string(),
                max_size_gb: 50.0,
            },
            network: NetworkConfig {
                timeout_secs: 30,
                retries: 3,
            },
            runtimes: RuntimesConfig {
                bun: "auto".to_string(),
                uv: "auto".to_string(),
                flutter: "auto".to_string(),
            },
            registries: RegistriesConfig::default(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            toml::from_str(&content).map_err(|e| anyhow!("Failed to parse config.toml: {}", e))
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let home = home_dir().ok_or_else(|| anyhow!("Could not determine home directory"))?;
        Ok(home.join(".offpkg/config.toml"))
    }

    pub fn cache_path(&self) -> PathBuf {
        // OFFPKG_CACHE_DIR env var overrides config
        if let Ok(override_dir) = env::var("OFFPKG_CACHE_DIR") {
            return PathBuf::from(override_dir);
        }
        PathBuf::from(&self.cache.path)
    }

    pub fn custom_stacks_dir(&self) -> PathBuf {
        if let Ok(override_dir) = env::var("OFFPKG_STACKS_DIR") {
            return PathBuf::from(override_dir);
        }
        let home = home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        home.join(".offpkg").join("stacks")
    }

    pub fn templates_dir(&self) -> PathBuf {
        if let Ok(override_dir) = env::var("OFFPKG_TEMPLATES_DIR") {
            return PathBuf::from(override_dir);
        }
        let home = home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        home.join(".offpkg").join("templates")
    }

    pub fn docs_dir(&self) -> PathBuf {
        let home = home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        home.join(".offpkg").join("docs")
    }
}

/// Cross-platform home directory (avoids deprecated std::env::home_dir)
pub fn home_dir() -> Option<PathBuf> {
    if let Ok(offpkg_home) = env::var("OFFPKG_HOME") {
        return Some(PathBuf::from(offpkg_home));
    }
    env::var("HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(|| env::var("USERPROFILE").ok().map(PathBuf::from))
}
