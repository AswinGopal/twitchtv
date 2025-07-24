use anyhow::{Result, anyhow};
use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use which::which;

pub struct Streamlink {
    exe: PathBuf,
}

impl Streamlink {
    pub fn discover() -> Result<Self> {
        if let Ok(p) = std::env::var("STREAMLINK") {
            return Ok(Self { exe: p.into() });
        }

        if let Some(home) = std::env::var_os("HOME") {
            let cand = Path::new(&home).join("myenv/streamlink/bin/streamlink");
            if cand.exists() {
                return Ok(Self { exe: cand });
            }
        }

        let exe = which("streamlink")
            .map_err(|_| anyhow!("Streamlink not found – install it or set $STREAMLINK"))?;
        Ok(Self { exe })
    }

    pub fn exe(&self) -> &Path {
        &self.exe
    }

    pub fn is_live(&self, channel: &str) -> bool {
        Command::new(&self.exe)
            .arg("--json")
            .arg("--loglevel")
            .arg("error")
            .arg(format!("twitch.tv/{channel}"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    pub fn qualities(&self, channel: &str) -> Vec<String> {
        let out = Command::new(&self.exe)
            .arg("--json")
            .arg("--loglevel")
            .arg("error")
            .arg(format!("twitch.tv/{channel}"))
            .output();

        if let Ok(o) = out {
            if o.status.success() {
                if let Ok(json) = serde_json::from_slice::<Value>(&o.stdout) {
                    if let Some(obj) = json.get("streams").and_then(|s| s.as_object()) {
                        let mut list: Vec<String> = obj
                            .keys()
                            .filter(|k| {
                                // only keep actual resolutions
                                !matches!(k.as_str(), "best" | "worst" | "audio_only")
                            })
                            .cloned()
                            .collect();

                        list.sort_by(|a, b| b.cmp(a)); // descending sort
                        return list;
                    }
                }
            }
        }

        vec![] // fallback
    }
}
