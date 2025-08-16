use chrono::{Duration as ChronoDuration, prelude::*};
use cron::Schedule;
use std::{error::Error, str::FromStr, time::Duration};

/// Calculates the duration until the next occurrence of a cron expression.
///
/// # Arguments
///
/// * `cron_expr` - A string slice containing the cron expression.
///
/// # Returns
///
/// A `Result` which is either:
/// * `Ok(Duration)`: The duration to wait until the next occurrence.
/// * `Err(Box<dyn Error>)`: An error if the cron expression is invalid or the time
///   calculation fails.
pub fn get_next_wait_duration(cron_expr: &str) -> Result<Duration, Box<dyn Error>> {
    let schedule = Schedule::from_str(cron_expr)?;
    let now = Local::now();

    let next_occurrence = schedule
        .upcoming(Local)
        .next()
        .ok_or("Could not find the next occurrence of the cron schedule.")?;

    let chrono_duration: ChronoDuration = next_occurrence.signed_duration_since(now);

    let wait_duration = Duration::new(
        chrono_duration.num_seconds() as u64,
        chrono_duration.num_nanoseconds().unwrap_or(0) as u32,
    );

    Ok(wait_duration)
}
