use clap::{Parser, Subcommand};
use std::{error::Error, thread};

use cronwait::get_next_wait_duration;

/// A simple CLI tool to wait for or display the time until the next cron schedule.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Prints the time in seconds until the next cron schedule.
    Next {
        /// The cron expression to check.
        cron_expr: String,

        /// The decimal precision for the output.
        #[clap(short, long, value_parser, default_value_t = 0)]
        precision: usize,
    },
    /// Waits for the time until the next cron schedule.
    Wait {
        /// The cron expression to wait for.
        cron_expr: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Next {
            cron_expr,
            precision,
        } => {
            let duration = get_next_wait_duration(&cron_expr)?;

            let total_seconds = duration.as_secs_f64();

            println!("{:.precision$}", total_seconds);
        }
        Commands::Wait { cron_expr } => {
            let duration = get_next_wait_duration(&cron_expr)?;
            eprintln!("Waiting for {:.2} seconds...", duration.as_secs_f64());
            thread::sleep(duration);

            println!("Done.");
        }
    }

    Ok(())
}

