use clap::Parser;
use log::info;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

use anyhow::Result;

// TODO: BufReader instead of read_to_string()
fn main() -> Result<()> {
    let args = Cli::parse();

    // Make an env logger with ability to increase our logging
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("Cooper's amazing base CLI");

    Ok(())
}
