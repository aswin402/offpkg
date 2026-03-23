use anyhow::{anyhow, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use crate::config::Config;
use crate::db::Package;

#[derive(Clone)]
pub struct Cache {
    config: Config,
}

impl Cache {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Returns the expected cache file path for a given runtime/name/version.
    /// Scoped npm packages like @vitejs/plugin-react are flattened to
    /// __vitejs__plugin-react to avoid creating subdirectories.
    pub fn path_for(&self, runtime: &str, name: &str, version: &str) -> PathBuf {
        let ext = match runtime {
            "uv"      => "whl",
            "flutter" => "tar.gz",
            _         => "tgz",
        };
        // Flatten scoped package names: @scope/name -> __scope__name
        let safe_name = name
            .replace('@', "__")
            .replace('/', "__");
        self.config.cache_path()
            .join(runtime)
            .join(format!("{}@{}.{}", safe_name, version, ext))
    }

    /// SHA-256 checksum verification.
    pub fn verify_checksum(&self, path: &Path, expected: &str) -> Result<bool> {
        let checksum = compute_sha256(path)?;
        Ok(checksum == expected)
    }

    pub fn size_bytes(&self, path: &Path) -> Result<u64> {
        Ok(fs::metadata(path)?.len())
    }

    pub fn exists(&self, pkg: &Package) -> bool {
        Path::new(&pkg.cache_path).exists()
    }

    /// Create the per-runtime cache subdirectory.
    pub fn ensure_dir(&self, runtime: &str) -> Result<()> {
        let path = self.config.cache_path().join(runtime);
        fs::create_dir_all(path)?;
        Ok(())
    }

    /// Async HTTP download — must be called from an async context.
    /// Using reqwest::blocking inside #[tokio::main] causes a panic.
    pub async fn download_to(&self, url: &str, dest: &Path) -> Result<u64> {
        let response = reqwest::get(url)
            .await
            .map_err(|e| anyhow!("Download failed for {}: {}", url, e))?;

        if !response.status().is_success() {
            return Err(anyhow!("HTTP {} for {}", response.status(), url));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| anyhow!("Failed to read response body: {}", e))?;

        let mut file = fs::File::create(dest)
            .map_err(|e| anyhow!("Cannot create file {:?}: {}", dest, e))?;

        file.write_all(&bytes)?;
        Ok(bytes.len() as u64)
    }
}

/// Standalone SHA-256 helper reused by adapters.
pub fn compute_sha256(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path)
        .map_err(|e| anyhow!("Cannot open {:?} for checksum: {}", path, e))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}