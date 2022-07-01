use clap::Parser;
use log::{debug, info};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

use anyhow::{Context, Result};

// TODO: BufReader instead of read_to_string()
fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    // Make an env logger with ability to increase our logging
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("Cooper's amazing base CLI");

    Ok(())
}
