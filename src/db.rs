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
        match stmt.query_row(params![name, version, runtime], |row| row_to_package(row)) {
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
            let rows = stmt.query_map([rt], |row| row_to_package(row))?;
            for row in rows {
                results.push(row.map_err(|e| anyhow!("{}", e))?);
            }
        } else {
            let mut stmt = self.conn.prepare(
                "SELECT id, name, version, runtime, cache_path, checksum, size_bytes, cached_at
                 FROM packages ORDER BY cached_at DESC",
            )?;
            let rows = stmt.query_map([], |row| row_to_package(row))?;
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
        let rows = stmt.query_map(params![name, runtime], |row| row_to_package(row))?;
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
