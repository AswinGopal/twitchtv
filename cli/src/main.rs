use anyhow::Result;
use clap::Parser;
use twitchtv_core::{App, player};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "mpv")]
    player: String,

    #[arg(short = 's', long = "stream", value_name = "CHANNEL")]
    stream: Option<String>, // ✅ New: allow passing a stream name
}

fn main() -> Result<()> {
    let args = Args::parse();

    let sl = twitchtv_core::streamlink::Streamlink::discover()?;
    let app = App::new(player::Mpv {
        streamlink_path: sl.exe().to_path_buf(),
    })?;

    if let Some(stream_name) = args.stream {
        return app.run_with_stream(stream_name);
    }

    app.run()
}
