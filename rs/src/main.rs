use anyhow::Result;
use clap::Parser;
use clap::ValueEnum;
use tracing::debug;
use tracing::info;
use tracing_glog::Glog;
use tracing_glog::GlogFields;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;

const LONG_ABOUT: &str = "This is a base CLI to use with new rust projects";

// This enum can be used to add `log-level` option to CLI binaries.
#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum PublicLogLevels {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<PublicLogLevels> for LevelFilter {
    fn from(public_level: PublicLogLevels) -> Self {
        match public_level {
            PublicLogLevels::Error => LevelFilter::ERROR,
            PublicLogLevels::Warn => LevelFilter::WARN,
            PublicLogLevels::Info => LevelFilter::INFO,
            PublicLogLevels::Debug => LevelFilter::DEBUG,
            PublicLogLevels::Trace => LevelFilter::TRACE,
        }
    }
}

/// Clap CLI Args struct with metadata in help output
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct Cli {
    /// Number that brings you luck
    #[clap(short, long, value_parser, default_value_t = 69)]
    a_lucky_number: u8,

    /// Adjust the console log-level
    #[arg(long, short, value_enum, ignore_case = true, default_value = "Info")]
    log_level: PublicLogLevels,
}

fn setup_logging(log_filter_level: LevelFilter) {
    let fmt = fmt::Layer::default()
        .with_writer(std::io::stderr)
        .event_format(Glog::default().with_timer(tracing_glog::LocalTime::default()))
        .fmt_fields(GlogFields)
        .with_filter(log_filter_level);

    let subscriber = Registry::default().with(fmt);
    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set global tracing subscriber");
}

fn main() -> Result<()> {
    let args = Cli::parse();
    setup_logging(args.log_level.into());

    info!("Cooper's amazing base CLI. Args: {:?}", args);
    debug!("Debug logging enabled");

    Ok(())
}
