//! Conta Configuration
use anyhow::{anyhow, Result};
use core::str::FromStr;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};
use toml_edit::Document;

/// Conta configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// The packages should be kept in order by the
    /// dependency graph.
    pub packages: Vec<String>,
}

impl Config {
    /// Create a new configuration from path.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        toml::from_str(&fs::read_to_string(path)?).map_err(|e| e.into())
    }

    /// Create a new configuration from cargo manifest.
    pub fn from_manifest(manifest: impl AsRef<Path>) -> Result<Self> {
        let doc = Document::from_str(&fs::read_to_string(manifest)?)?;
        let table = doc["workspace"]["metadata"]["conta"]
            .as_table()
            .ok_or_else(|| anyhow!("No conta metadata"))?;
        toml::from_str(&table.to_string()).map_err(|e| e.into())
    }

    /// Create a new configuration from optional path.
    pub fn from_optional(path: Option<impl AsRef<Path>>) -> Result<Self> {
        if let Some(path) = path {
            if path.as_ref().exists() {
                return Self::from_path(path);
            }
        }

        let cwd = env::current_dir()?;
        let conta = cwd.join("Conta.toml");
        if conta.exists() {
            Self::from_path(conta)
        } else {
            Self::from_manifest(cwd.join("Cargo.toml"))
        }
    }
}

#[test]
fn from_manifest() -> Result<()> {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let config = Config::from_manifest(format!("{manifest}/../Cargo.toml"))?;
    assert_eq!(config.packages, vec!["ccli".to_string(), "conta".into()]);
    Ok(())
}
