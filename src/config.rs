use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub cache: CacheConfig,
    pub network: NetworkConfig,
    pub runtimes: RuntimesConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CacheConfig {
    pub path: String,
    pub max_size_gb: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NetworkConfig {
    pub timeout_secs: u64,
    pub retries: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RuntimesConfig {
    pub bun: String,
    pub uv: String,
    pub flutter: String,
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

    fn config_path() -> Result<PathBuf> {
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
}

/// Cross-platform home directory (avoids deprecated std::env::home_dir)
fn home_dir() -> Option<PathBuf> {
    env::var("HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(|| env::var("USERPROFILE").ok().map(PathBuf::from))
}
