use clap::Parser;
use clap_verbosity_flag::InfoLevel;
use log::info;

const LONG_ABOUT: &str = "This is a base CLI to use with new rust projects";

/// Clap CLI Args struct with metadata in help output
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity<InfoLevel>,
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
