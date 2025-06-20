use anyhow::Result;
use clap::Parser;
use tracing::debug;
use tracing::error;
use tracing::info;

const LONG_ABOUT: &str = "This is a base CLI to use with new rust projects";

#[derive(Debug, Parser)]
struct LuckyNumberArgs {
    /// Lucky numbers to select from
    #[clap()]
    numbers: Vec<i64>,

    /// Only consider odd numbers lucky
    #[clap(short)]
    odd: bool,
}

/// Clap CLI Args struct with metadata in help output
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct Args {
    /// Number that brings you luck
    #[clap(short, long, value_parser, default_value_t = 69)]
    a_lucky_number: i64,

    #[clap(subcommand)]
    command: Command,

    /// Adjust the console log-level
    #[arg(long, short, value_enum, ignore_case = true, default_value = "Info")]
    log_level: rs::LogLevels,
}

#[derive(Debug, Parser)]
enum Command {
    #[clap(about = "Print your lucky number")]
    LuckyNumber(LuckyNumberArgs),
}

fn main() -> Result<()> {
    let args = Args::parse();
    rs::setup_logging(args.log_level.into());

    info!("Cooper's amazing base CLI. Args: {:?}", args);
    debug!("Debug logging enabled");

    match args.command {
        Command::LuckyNumber(ln_args) => {
            if ln_args.numbers.contains(&args.a_lucky_number) {
                println!("Fuck yeah, {} is always lucky!", args.a_lucky_number);
                return Ok(());
            }
            let a_lucky_number = ln_args.numbers.clone().pop();
            match a_lucky_number {
                Some(aln) => {
                    if ln_args.odd && aln % 2 != 0 {
                        println!("{aln} is odd and lucky!")
                    } else {
                        println!("{aln} is lucky!");
                    }
                }
                None => {
                    error!("No fucking lucky number ... Give me some numbers!");
                }
            }
        }
    }

    Ok(())
}
