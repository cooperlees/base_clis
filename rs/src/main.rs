use anyhow::Result;
use clap::Parser;
use tracing::debug;
use tracing::info;

const LONG_ABOUT: &str = "This is a base CLI to use with new rust projects";

/// Clap CLI Args struct with metadata in help output
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct Cli {
    /// Number that brings you luck
    #[clap(short, long, value_parser, default_value_t = 69)]
    a_lucky_number: u8,

    /// Adjust the console log-level
    #[arg(long, short, value_enum, ignore_case = true, default_value = "Info")]
    log_level: rs::LogLevels,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    rs::setup_logging(args.log_level.into());

    info!("Cooper's amazing base CLI. Args: {:?}", args);
    debug!("Debug logging enabled");

    Ok(())
}
