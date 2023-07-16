use anyhow::Result;
use clap::Parser;
use tracing::debug;
use tracing::info;
use tracing_glog::Glog;
use tracing_glog::GlogFields;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;

const LONG_ABOUT: &str = "This is a base CLI to use with new rust projects";

/// Clap CLI Args struct with metadata in help output
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct Cli {
    /// Enable debug logging
    #[clap(short, long)]
    debug: bool,

    /// Number that brings you luck
    #[clap(short, long, value_parser, default_value_t = 69)]
    lucky_number: u8,
}

fn setup_logging(debug: bool) {
    let log_filter_level = match debug {
        true => LevelFilter::DEBUG,
        false => LevelFilter::INFO,
    };
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
    setup_logging(args.debug);

    info!("Cooper's amazing base CLI. Args: {:?}", args);
    debug!("Debug logging enabled");

    Ok(())
}
