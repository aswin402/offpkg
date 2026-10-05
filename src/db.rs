use crate::config::Config;
use anyhow::{anyhow, Result};
use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub runtime: String,
    pub cache_path: String,
    pub checksum: String,
    pub size_bytes: Option<i64>,
    pub cached_at: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(config: &Config) -> Result<Self> {
        let db_path = config.cache_path().join("offpkg.db");
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&db_path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS packages (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                name        TEXT NOT NULL,
                version     TEXT NOT NULL,
                runtime     TEXT NOT NULL CHECK(runtime IN ('bun','uv','flutter')),
                cache_path  TEXT NOT NULL,
                checksum    TEXT NOT NULL,
                size_bytes  INTEGER,
                cached_at   DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(name, version, runtime)
            );
            CREATE TABLE IF NOT EXISTS config (
                key   TEXT PRIMARY KEY,
                value TEXT
            );",
        )?;
        let integrity: String = conn.query_row("PRAGMA integrity_check", [], |row| row.get(0))?;
        if integrity != "ok" {
            return Err(anyhow!("DB integrity check failed: {}", integrity));
        }
        Ok(Self { conn })
    }

    pub fn insert_package(&self, pkg: &Package) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO packages
                (name, version, runtime, cache_path, checksum, size_bytes, cached_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                pkg.name,
                pkg.version,
                pkg.runtime,
                pkg.cache_path,
                pkg.checksum,
                pkg.size_bytes,
                pkg.cached_at
            ],
        )?;
        Ok(())
    }

    pub fn get_package(&self, name: &str, version: &str, runtime: &str) -> Result<Option<Package>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, version, runtime, cache_path, checksum, size_bytes, cached_at
             FROM packages WHERE name = ?1 AND version = ?2 AND runtime = ?3",
        )?;
        match stmt.query_row(params![name, version, runtime], row_to_package) {
            Ok(pkg) => Ok(Some(pkg)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(anyhow!("DB query error: {}", e)),
        }
    }

    pub fn list_packages(&self, runtime: Option<&str>) -> Result<Vec<Package>> {
        let mut results: Vec<Package> = Vec::new();
        if let Some(rt) = runtime {
            let mut stmt = self.conn.prepare(
                "SELECT id, name, version, runtime, cache_path, checksum, size_bytes, cached_at
                 FROM packages WHERE runtime = ?1 ORDER BY cached_at DESC",
            )?;
            let rows = stmt.query_map([rt], row_to_package)?;
            for row in rows {
                results.push(row.map_err(|e| anyhow!("{}", e))?);
            }
        } else {
            let mut stmt = self.conn.prepare(
                "SELECT id, name, version, runtime, cache_path, checksum, size_bytes, cached_at
                 FROM packages ORDER BY cached_at DESC",
            )?;
            let rows = stmt.query_map([], row_to_package)?;
            for row in rows {
                results.push(row.map_err(|e| anyhow!("{}", e))?);
            }
        }
        Ok(results)
    }

    /// Find all cached versions of a package for a given runtime
    pub fn find_packages(&self, name: &str, runtime: &str) -> Result<Vec<Package>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, version, runtime, cache_path, checksum, size_bytes, cached_at
             FROM packages WHERE name = ?1 AND runtime = ?2 ORDER BY cached_at DESC",
        )?;
        let rows = stmt.query_map(params![name, runtime], row_to_package)?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| anyhow!("{}", e))?);
        }
        Ok(results)
    }

    /// Delete a package record from the DB by name + runtime (all versions)
    pub fn delete_package(&self, name: &str, runtime: &str) -> Result<usize> {
        let count = self.conn.execute(
            "DELETE FROM packages WHERE name = ?1 AND runtime = ?2",
            params![name, runtime],
        )?;
        Ok(count)
    }

    /// Delete a specific version
    pub fn delete_package_version(
        &self,
        name: &str,
        version: &str,
        runtime: &str,
    ) -> Result<usize> {
        let count = self.conn.execute(
            "DELETE FROM packages WHERE name = ?1 AND version = ?2 AND runtime = ?3",
            params![name, version, runtime],
        )?;
        Ok(count)
    }

    pub fn count_packages(&self) -> Result<i64> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM packages", [], |row| row.get(0))?;
        Ok(count)
    }
}

fn row_to_package(row: &rusqlite::Row) -> SqlResult<Package> {
    Ok(Package {
        id: row.get(0)?,
        name: row.get(1)?,
        version: row.get(2)?,
        runtime: row.get(3)?,
        cache_path: row.get(4)?,
        checksum: row.get(5)?,
        size_bytes: row.get(6)?,
        cached_at: row.get(7)?,
    })
}

impl Clone for Database {
    fn clone(&self) -> Self {
        let path = self.conn.path().expect("Failed to get db path");
        let conn = Connection::open(path).expect("Failed to clone db connection");
        Self { conn }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_db() -> (Database, std::path::PathBuf) {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let test_dir = std::env::temp_dir().join(format!("offpkg_db_test_{}", nanos));
        std::fs::create_dir_all(&test_dir).unwrap();

        let mut config = Config::default();
        config.cache.path = test_dir.to_string_lossy().to_string();

        let db = Database::open(&config).unwrap();
        (db, test_dir)
    }

    #[test]
    fn test_db_lifecycle_and_crud() {
        let (db, test_dir) = setup_test_db();

        // 1. Initial count should be 0
        assert_eq!(db.count_packages().unwrap(), 0);

        // 2. Insert packages
        let pkg1 = Package {
            id: 0,
            name: "react".to_string(),
            version: "19.0.0".to_string(),
            runtime: "bun".to_string(),
            cache_path: "/tmp/react@19.0.0.tgz".to_string(),
            checksum: "abc123sha".to_string(),
            size_bytes: Some(1024),
            cached_at: "2026-10-05T00:00:00Z".to_string(),
        };

        let pkg2 = Package {
            id: 0,
            name: "react".to_string(),
            version: "19.1.0".to_string(),
            runtime: "bun".to_string(),
            cache_path: "/tmp/react@19.1.0.tgz".to_string(),
            checksum: "def456sha".to_string(),
            size_bytes: Some(2048),
            cached_at: "2026-10-05T01:00:00Z".to_string(),
        };

        let pkg3 = Package {
            id: 0,
            name: "fastapi".to_string(),
            version: "0.110.0".to_string(),
            runtime: "uv".to_string(),
            cache_path: "/tmp/fastapi@0.110.0.whl".to_string(),
            checksum: "ghi789sha".to_string(),
            size_bytes: Some(4096),
            cached_at: "2026-10-05T02:00:00Z".to_string(),
        };

        db.insert_package(&pkg1).unwrap();
        db.insert_package(&pkg2).unwrap();
        db.insert_package(&pkg3).unwrap();

        // 3. Count packages
        assert_eq!(db.count_packages().unwrap(), 3);

        // 4. Duplicate insert should be ignored due to UNIQUE constraint
        db.insert_package(&pkg1).unwrap();
        assert_eq!(db.count_packages().unwrap(), 3);

        // 5. Get package
        let found = db.get_package("react", "19.0.0", "bun").unwrap();
        assert!(found.is_some());
        let found_pkg = found.unwrap();
        assert_eq!(found_pkg.name, "react");
        assert_eq!(found_pkg.version, "19.0.0");
        assert_eq!(found_pkg.checksum, "abc123sha");

        let not_found = db.get_package("react", "20.0.0", "bun").unwrap();
        assert!(not_found.is_none());

        // 6. List packages (filtered and unfiltered)
        let all_pkgs = db.list_packages(None).unwrap();
        assert_eq!(all_pkgs.len(), 3);

        let bun_pkgs = db.list_packages(Some("bun")).unwrap();
        assert_eq!(bun_pkgs.len(), 2);

        let uv_pkgs = db.list_packages(Some("uv")).unwrap();
        assert_eq!(uv_pkgs.len(), 1);

        // 7. Find packages by name and runtime
        let react_versions = db.find_packages("react", "bun").unwrap();
        assert_eq!(react_versions.len(), 2);

        // 8. Delete package version
        let deleted = db.delete_package_version("react", "19.0.0", "bun").unwrap();
        assert_eq!(deleted, 1);
        assert_eq!(db.count_packages().unwrap(), 2);

        // 9. Delete entire package across versions
        let deleted_react = db.delete_package("react", "bun").unwrap();
        assert_eq!(deleted_react, 1);
        assert_eq!(db.count_packages().unwrap(), 1);

        let remaining = db.list_packages(None).unwrap();
        assert_eq!(remaining[0].name, "fastapi");

        // Clean up
        let _ = std::fs::remove_dir_all(test_dir);
    }
}
