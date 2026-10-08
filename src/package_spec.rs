use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageSpec {
    pub name: String,
    pub version_req: Option<String>,
}

impl PackageSpec {
    pub fn new(name: impl Into<String>, version_req: Option<String>) -> Self {
        Self {
            name: name.into(),
            version_req,
        }
    }

    /// Parse a package string according to the ecosystem/runtime rules.
    ///
    /// Examples:
    /// - Bun/npm:
    ///     "react" -> ("react", None)
    ///     "react@18.2.0" -> ("react", Some("18.2.0"))
    ///     "react@^18.0.0" -> ("react", Some("^18.0.0"))
    ///     "@types/react" -> ("@types/react", None)
    ///     "@types/react@18.2.0" -> ("@types/react", Some("18.2.0"))
    ///
    /// - uv/PyPI:
    ///     "requests" -> ("requests", None)
    ///     "requests==2.31.0" -> ("requests", Some("2.31.0"))
    ///     "requests>=2.30.0" -> ("requests", Some(">=2.30.0"))
    ///     "requests@2.31.0" -> ("requests", Some("2.31.0"))
    ///
    /// - Flutter/pub.dev:
    ///     "provider" -> ("provider", None)
    ///     "provider:^6.0.0" -> ("provider", Some("^6.0.0"))
    ///     "provider@6.0.0" -> ("provider", Some("6.0.0"))
    ///     "provider:6.0.0" -> ("provider", Some("6.0.0"))
    pub fn parse(raw: &str, runtime: &str) -> Result<Self> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("Package name cannot be empty"));
        }

        match runtime {
            "bun" | "npm" => Self::parse_npm(trimmed),
            "uv" | "python" | "pypi" => Self::parse_pypi(trimmed),
            "flutter" | "dart" | "pub" => Self::parse_flutter(trimmed),
            _ => Self::parse_generic(trimmed),
        }
    }

    fn parse_npm(raw: &str) -> Result<Self> {
        if raw.starts_with('@') {
            // Scoped package e.g. @types/react or @types/react@18.2.0
            if let Some(second_at) = raw[1..].find('@') {
                let name = &raw[..=second_at];
                let ver = &raw[second_at + 2..];
                if ver.is_empty() {
                    return Err(anyhow!("Empty version in scoped package: '{}'", raw));
                }
                Ok(Self::new(name, Some(ver.to_string())))
            } else {
                Ok(Self::new(raw, None))
            }
        } else if let Some((name, ver)) = raw.split_once('@') {
            if name.is_empty() {
                return Err(anyhow!("Invalid package name: '{}'", raw));
            }
            if ver.is_empty() {
                return Err(anyhow!("Empty version in package: '{}'", raw));
            }
            Ok(Self::new(name, Some(ver.to_string())))
        } else {
            Ok(Self::new(raw, None))
        }
    }

    fn parse_pypi(raw: &str) -> Result<Self> {
        // Handle ==, >=, <=, ~=, or @
        for op in ["==", ">=", "<=", "~="] {
            if let Some((name, ver)) = raw.split_once(op) {
                let name = name.trim();
                let ver = ver.trim();
                if name.is_empty() || ver.is_empty() {
                    return Err(anyhow!("Invalid PyPI package specification: '{}'", raw));
                }
                return Ok(Self::new(name, Some(ver.to_string())));
            }
        }

        if let Some((name, ver)) = raw.split_once('@') {
            let name = name.trim();
            let ver = ver.trim();
            if name.is_empty() || ver.is_empty() {
                return Err(anyhow!("Invalid PyPI package specification: '{}'", raw));
            }
            return Ok(Self::new(name, Some(ver.to_string())));
        }

        Ok(Self::new(raw, None))
    }

    fn parse_flutter(raw: &str) -> Result<Self> {
        // Handle : or @ (e.g. provider:^6.0.0 or provider@6.0.0)
        if let Some((name, ver)) = raw.split_once(':') {
            let name = name.trim();
            let ver = ver.trim();
            if name.is_empty() || ver.is_empty() {
                return Err(anyhow!("Invalid Flutter package specification: '{}'", raw));
            }
            Ok(Self::new(name, Some(ver.to_string())))
        } else if let Some((name, ver)) = raw.split_once('@') {
            let name = name.trim();
            let ver = ver.trim();
            if name.is_empty() || ver.is_empty() {
                return Err(anyhow!("Invalid Flutter package specification: '{}'", raw));
            }
            Ok(Self::new(name, Some(ver.to_string())))
        } else {
            Ok(Self::new(raw, None))
        }
    }

    fn parse_generic(raw: &str) -> Result<Self> {
        if let Some((name, ver)) = raw.split_once('@') {
            Ok(Self::new(name.trim(), Some(ver.trim().to_string())))
        } else {
            Ok(Self::new(raw.trim(), None))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_npm_specs() {
        let p1 = PackageSpec::parse("react", "bun").unwrap();
        assert_eq!(p1.name, "react");
        assert_eq!(p1.version_req, None);

        let p2 = PackageSpec::parse("react@18.2.0", "bun").unwrap();
        assert_eq!(p2.name, "react");
        assert_eq!(p2.version_req, Some("18.2.0".to_string()));

        let p3 = PackageSpec::parse("@types/react", "bun").unwrap();
        assert_eq!(p3.name, "@types/react");
        assert_eq!(p3.version_req, None);

        let p4 = PackageSpec::parse("@types/react@^18.0.0", "bun").unwrap();
        assert_eq!(p4.name, "@types/react");
        assert_eq!(p4.version_req, Some("^18.0.0".to_string()));
    }

    #[test]
    fn test_parse_pypi_specs() {
        let p1 = PackageSpec::parse("requests", "uv").unwrap();
        assert_eq!(p1.name, "requests");
        assert_eq!(p1.version_req, None);

        let p2 = PackageSpec::parse("requests==2.31.0", "uv").unwrap();
        assert_eq!(p2.name, "requests");
        assert_eq!(p2.version_req, Some("2.31.0".to_string()));

        let p3 = PackageSpec::parse("requests>=2.28.0", "uv").unwrap();
        assert_eq!(p3.name, "requests");
        assert_eq!(p3.version_req, Some("2.28.0".to_string()));

        let p4 = PackageSpec::parse("fastapi@0.110.0", "uv").unwrap();
        assert_eq!(p4.name, "fastapi");
        assert_eq!(p4.version_req, Some("0.110.0".to_string()));
    }

    #[test]
    fn test_parse_flutter_specs() {
        let p1 = PackageSpec::parse("provider", "flutter").unwrap();
        assert_eq!(p1.name, "provider");
        assert_eq!(p1.version_req, None);

        let p2 = PackageSpec::parse("provider:^6.0.0", "flutter").unwrap();
        assert_eq!(p2.name, "provider");
        assert_eq!(p2.version_req, Some("^6.0.0".to_string()));

        let p3 = PackageSpec::parse("dio@5.4.0", "flutter").unwrap();
        assert_eq!(p3.name, "dio");
        assert_eq!(p3.version_req, Some("5.4.0".to_string()));
    }
}
