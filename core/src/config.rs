use anyhow::{Result, anyhow, bail};
use directories::ProjectDirs;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct Config {
    pub channels: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        if let Ok(p) = std::env::var("TWITCHTV_CONFIG") {
            return Self::load_from(PathBuf::from(p));
        }

        if let Some(proj) = ProjectDirs::from("com", "example", "twitchtv") {
            let p = proj.config_dir().join("channels.toml");
            if p.exists() {
                return Self::load_from(p);
            }
        }

        if Path::new("channels.toml").exists() {
            return Self::load_from("channels.toml");
        }

        bail!(
            "channels.toml not found.
  • Put it at ~/.config/twitchtv/channels.toml
  • or set TWITCHTV_CONFIG=/path/to/channels.toml"
        )
    }

    fn load_from<P: Into<PathBuf>>(p: P) -> Result<Self> {
        let path = p.into();
        let data = std::fs::read_to_string(&path)
            .map_err(|e| anyhow!("Failed to read {}: {e}", path.display()))?;
        Ok(toml::from_str(&data)?)
    }
}
