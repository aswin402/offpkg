use crate::config::Config;
use crate::db::Package;
use anyhow::{anyhow, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

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
            "uv" => "whl",
            "flutter" => "tar.gz",
            _ => "tgz",
        };
        // Flatten scoped package names: @scope/name -> __scope__name
        let safe_name = name.replace(['@', '/'], "__");
        self.config
            .cache_path()
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

        let mut file =
            fs::File::create(dest).map_err(|e| anyhow!("Cannot create file {:?}: {}", dest, e))?;

        file.write_all(&bytes)?;
        Ok(bytes.len() as u64)
    }
}

/// Standalone SHA-256 helper reused by adapters.
pub fn compute_sha256(path: &Path) -> Result<String> {
    let mut file =
        fs::File::open(path).map_err(|e| anyhow!("Cannot open {:?} for checksum: {}", path, e))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_cache_path_for_runtimes() {
        let config = Config::default();
        let cache = Cache::new(config.clone());
        let expected_base = config.cache_path();

        let bun_path = cache.path_for("bun", "react", "19.2.0");
        assert_eq!(bun_path, expected_base.join("bun/react@19.2.0.tgz"));

        let uv_path = cache.path_for("uv", "requests", "2.31.0");
        assert_eq!(uv_path, expected_base.join("uv/requests@2.31.0.whl"));

        let flutter_path = cache.path_for("flutter", "dio", "5.4.0");
        assert_eq!(flutter_path, expected_base.join("flutter/dio@5.4.0.tar.gz"));
    }

    #[test]
    fn test_scoped_package_flattening() {
        let config = Config::default();
        let cache = Cache::new(config.clone());
        let expected_base = config.cache_path();

        let scoped_path = cache.path_for("bun", "@vitejs/plugin-react", "4.2.1");
        assert_eq!(
            scoped_path,
            expected_base.join("bun/__vitejs__plugin-react@4.2.1.tgz")
        );

        let types_path = cache.path_for("bun", "@types/node", "20.11.0");
        assert_eq!(
            types_path,
            expected_base.join("bun/__types__node@20.11.0.tgz")
        );
    }

    #[test]
    fn test_compute_and_verify_sha256() {
        let dir = std::env::temp_dir().join("offpkg_test_cache");
        fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("test_sha256.txt");
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(b"offpkg cache checksum test").unwrap();
        drop(file);

        let hash = compute_sha256(&file_path).unwrap();
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64);

        let config = Config::default();
        let cache = Cache::new(config);
        assert!(cache.verify_checksum(&file_path, &hash).unwrap());
        assert!(!cache
            .verify_checksum(&file_path, "invalid_checksum")
            .unwrap());

        let _ = fs::remove_file(file_path);
    }
}
