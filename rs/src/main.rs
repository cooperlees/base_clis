use clap::Parser;
use log::info;

/// Clap CLI Args struct
#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

use anyhow::Result;

fn main() -> Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("Cooper's amazing base CLI");

    Ok(())
}
