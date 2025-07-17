use anyhow::Result;
use std::process::Command;
use std::process::Stdio;

pub trait Player {
    fn play(&self, channel: &str, quality: &str) -> Result<()>;
}

pub struct Mpv {
    pub streamlink_path: std::path::PathBuf,
}

impl Player for Mpv {
    fn play(&self, channel: &str, quality: &str) -> Result<()> {
        Command::new(&self.streamlink_path)
            .arg("--twitch-low-latency")
            .arg("-p")
            .arg("mpv")
            .arg(format!("twitch.tv/{channel}"))
            .arg(quality)
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .status()?;
        Ok(())
    }
}
