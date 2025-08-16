use clap::Parser;
use std::{error::Error, thread};

use cronwait::get_next_wait_duration;

/// A simple CLI tool to wait for or display the time until the next cron schedule.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The cron expression to use.
    cron_expr: String,

    /// Print the time in seconds until the next cron schedule instead of waiting.
    /// An optional precision can be provided.
    #[clap(
        short = 'p',
        long,
        value_name = "PRECISION",
        num_args = 0..=1,
        default_missing_value = "0"
    )]
    print: Option<usize>,

    /// Suppress verbose messages.
    #[clap(short = 'q', long)]
    quiet: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::try_parse()?;

    let duration = get_next_wait_duration(&args.cron_expr.trim())?;

    if let Some(precision) = args.print {
        let total_seconds = duration.as_secs_f64();
        println!("{:.precision$}", total_seconds);
    } else {
        if !args.quiet {
            eprintln!("Waiting for {:.2} seconds...", duration.as_secs_f64());
        }
        thread::sleep(duration);
        if !args.quiet {
            println!("Done.");
        }
    }

    Ok(())
}
