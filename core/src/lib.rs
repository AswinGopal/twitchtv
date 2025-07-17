pub mod config;
pub mod player;
pub mod streamlink;

use anyhow::Result;
use dialoguer::Select;
use rayon::prelude::*;

pub struct App<P: player::Player + Sync> {
    cfg: config::Config,
    sl: streamlink::Streamlink,
    player: P,
}

impl<P: player::Player + Sync> App<P> {
    pub fn new(player: P) -> Result<Self> {
        Ok(Self {
            cfg: config::Config::load()?,
            sl: streamlink::Streamlink::discover()?,
            player,
        })
    }

    pub fn run_with_stream(&self, stream: String) -> Result<()> {
        println!("🔍 Getting stream info for '{stream}'...");
        
        if !self.sl.is_live(&stream) {
            println!("🚫 '{}' is not live right now.", stream);
            return Ok(());
        }

        let qualities = self.sl.qualities(&stream);
        if qualities.is_empty() {
            println!("⚠️ No qualities found for '{}'", stream);
            return Ok(());
        }

        let qidx = Select::new()
            .with_prompt("Select stream quality")
            .items(&qualities)
            .default(0)
            .interact()?;
        let quality = &qualities[qidx];

        println!("\nLaunching {stream} at {quality}…\n");
        self.player.play(&stream, quality)
    }

    pub fn run(&self) -> Result<()> {
        if self.cfg.channels.is_empty() {
            println!("Your channels list is empty.");
            return Ok(());
        }

        println!("Checking who is live…");
        let sl = &self.sl;
        let live: Vec<String> = self
            .cfg
            .channels
            .par_iter()
            .filter(|c| sl.is_live(c))
            .cloned()
            .collect();

        if live.is_empty() {
            println!("😴 No followed channels are currently live.");
            return Ok(());
        }

        let idx = Select::new()
            .with_prompt("Choose a live stream 📺")
            .items(&live)
            .interact()?;
        let channel = &live[idx];

        let qualities = self.sl.qualities(channel);
        let qidx = Select::new()
            .with_prompt("Select stream quality")
            .items(&qualities)
            .default(0)
            .interact()?;
        let quality = &qualities[qidx];

        println!("\nLaunching {channel} at {quality}…\n");
        self.player.play(channel, quality)
    }
}

